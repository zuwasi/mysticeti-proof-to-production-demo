BeginPackage["MysticetiModel`"];
QuorumThreshold::usage = "QuorumThreshold[f] gives 2 f + 1 for an equal-voting-power committee.";
CommitteeSize::usage = "CommitteeSize[f] gives 3 f + 1.";
HonestIntersectionLowerBound::usage = "HonestIntersectionLowerBound[n,q,b] gives Max[0,2 q-n-b].";
SafetyEnvelopeData::usage = "SafetyEnvelopeData[maxN] returns quorum/intersection boundary rows.";
GenerateMysticetiDAG::usage = "GenerateMysticetiDAG[params] creates a deterministic inspectable round DAG whose parents have distinct authorities.";
SupportedProposal::usage = "SupportedProposal[dag,block,slot] returns at most one proposal block for the validator+round slot. It traverses ancestors breadth-first, sorting each frontier by {round,validator,ID}, and deterministically selects the first proposal encountered.";
CertificateEvidence::usage = "CertificateEvidence[dag,proposal] returns r+2 certificate blocks whose r+1 parents contain quorum distinct authorities supporting proposal.";
DirectSlotDecision::usage = "DirectSlotDecision[dag,slot] applies the educational Section II-C direct commit/skip pattern to an explicit <|Validator,Round|> slot.";
SimulateMysticeti::usage = "SimulateMysticeti[params] runs a seeded uncalibrated synthetic sensitivity model.";
RunFaultSweep::usage = "RunFaultSweep[] compares crash counts and jitter in structured synthetic-sensitivity rows.";
DAGVisualization::usage = "DAGVisualization[dag] renders blocks styled by status and round.";
SafetyEnvelopePlot::usage = "SafetyEnvelopePlot[] plots explicit committee-size/intersection pairs grouped by Byzantine count.";
FaultFrontierPlot::usage = "FaultFrontierPlot[] plots synthetic sensitivity-model P95 latency.";
ProductionComparisonPlot::usage = "ProductionComparisonPlot[] imports and plots data/paper_production_benchmark.csv.";
ImportRustTrace::usage = "ImportRustTrace[path] imports a versioned mysticeti-twin JSON trace as an Association.";
ValidateRustTrace::usage = "ValidateRustTrace[trace] independently audits the Rust trace's schema, references, stake arithmetic, decisions, and recorded checks. This is conformance checking, not a Rust DAG reimplementation.";
RustTraceSummary::usage = "RustTraceSummary[trace] returns compact counts and stake thresholds from an imported Rust trace.";
RustFaultSweepPlot::usage = "RustFaultSweepPlot[path] plots commit count and rate versus packet loss from a Rust event-driven sweep CSV.";
RunValidationSuite::usage = "RunValidationSuite[] runs decision fixtures, authority-counting, DAG, replay, and CSV-transcription checks.";

Begin["`Private`"];
packageRoot = DirectoryName[DirectoryName[$InputFileName]];
QuorumThreshold[f_Integer?NonNegative] := 2 f + 1;
CommitteeSize[f_Integer?NonNegative] := 3 f + 1;
HonestIntersectionLowerBound[n_Integer, q_Integer, b_Integer] := Max[0, 2 q - n - b];
SafetyEnvelopeData[maxN_Integer : 40] := Flatten[Table[Table[<|"CommitteeSize" -> n, "Byzantine" -> b,
  "Quorum" -> Floor[2 n/3] + 1, "HonestIntersection" -> HonestIntersectionLowerBound[n, Floor[2 n/3] + 1, b],
  "Safe" -> HonestIntersectionLowerBound[n, Floor[2 n/3] + 1, b] > 0|>, {b, 0, Floor[(n - 1)/3] + 1}], {n, 4, maxN}], 1];

defaults = <|"F" -> 2, "Rounds" -> 7, "Seed" -> 42, "CrashCount" -> 0, "ByzantineCount" -> 0,
  "Equivocation" -> False, "BaseDelayMS" -> 120., "JitterMS" -> 35., "SyntheticSamples" -> 500|>;
blockOrder[b_] := {b["Round"], b["Validator"], b["ID"]};
blockByID[blocks_, id_] := FirstCase[blocks, b_ /; b["ID"] === id, Missing["NotFound"]];
slotSpec[slot_, blocks_] := Which[
  AssociationQ[slot] && KeyExistsQ[slot, "Validator"] && KeyExistsQ[slot, "Round"], KeyTake[slot, {"Validator", "Round"}],
  StringQ[slot], With[{b = blockByID[blocks, slot]}, If[MissingQ[b], Missing["NotFound"], KeyTake[b, {"Validator", "Round"}]]],
  True, Missing["NotFound"]];
authorityIDs[blocks_] := DeleteDuplicates[Lookup[blocks, "Validator"]];

GenerateMysticetiDAG[p_Association] := Module[{x = Join[defaults, p], f, n, q, rounds, crashes, byz, blocks = {}, prior,
    choices, parents, id, status, v, r, ids},
  SeedRandom[x["Seed"]]; f = x["F"]; n = CommitteeSize[f]; q = QuorumThreshold[f]; rounds = x["Rounds"];
  crashes = Range[Max[1, n - x["CrashCount"] + 1], n]; byz = Range[1, Min[n, x["ByzantineCount"]]];
  Do[Do[If[!MemberQ[crashes, v],
    prior = SortBy[Select[blocks, #["Round"] == r - 1 &], blockOrder];
    choices = First /@ GatherBy[prior, #["Validator"] &];
    parents = If[r == 1, {}, Lookup[Take[choices, UpTo[Min[q, Length[choices]]]], "ID"]];
    id = "r" <> ToString[r] <> "v" <> ToString[v]; status = If[MemberQ[byz, v], "Byzantine", "Honest"];
    AppendTo[blocks, <|"ID" -> id, "Validator" -> v, "Round" -> r, "Parents" -> parents, "Status" -> status,
      "Crashed" -> False, "Equivocation" -> False|>];
    If[TrueQ[x["Equivocation"]] && MemberQ[byz, v], AppendTo[blocks, <|"ID" -> id <> "e", "Validator" -> v,
      "Round" -> r, "Parents" -> Reverse[parents], "Status" -> "Equivocation", "Crashed" -> False, "Equivocation" -> True|>]]],
    {v, n}], {r, rounds}];
  ids = Lookup[blocks, "ID"];
  <|"Parameters" -> x, "CommitteeSize" -> n, "Quorum" -> q, "CrashedValidators" -> crashes,
    "EqualVotingPower" -> True, "Blocks" -> blocks,
    "Edges" -> Flatten[Table[parent -> b["ID"], {b, blocks}, {parent, b["Parents"]}], 1],
    "ValidParentReferences" -> AllTrue[Flatten[Lookup[blocks, "Parents"]], MemberQ[ids, #] &],
    "DistinctParentAuthorities" -> AllTrue[blocks, DuplicateFreeQ[Lookup[DeleteMissing[blockByID[blocks, #] & /@ #["Parents"]], "Validator"]] &]|>];

SupportedProposal[dag_Association, block_, slot_] := Module[{blocks = dag["Blocks"], b, ss, proposals, frontier, seen = {}, level, hits},
  b = If[AssociationQ[block], block, blockByID[blocks, block]]; ss = slotSpec[slot, blocks];
  If[MissingQ[b] || MissingQ[ss], Return[Missing["NotFound"]]];
  proposals = Select[blocks, #["Validator"] === ss["Validator"] && #["Round"] === ss["Round"] &];
  frontier = SortBy[DeleteMissing[blockByID[blocks, #] & /@ b["Parents"]], blockOrder];
  While[frontier =!= {},
    level = Select[frontier, !MemberQ[seen, #["ID"]] &]; seen = Join[seen, Lookup[level, "ID"]];
    hits = SortBy[Select[level, MemberQ[Lookup[proposals, "ID"], #["ID"]] &], blockOrder];
    If[hits =!= {}, Return[First[hits]]];
    frontier = SortBy[DeleteMissing[blockByID[blocks, #] & /@ Flatten[Lookup[level, "Parents"]]], blockOrder]];
  Missing["NoSupportedProposal"]];

CertificateEvidence[dag_Association, proposal_] := Module[{blocks = dag["Blocks"], p, slot, r, q, certs},
  p = If[AssociationQ[proposal], proposal, blockByID[blocks, proposal]]; If[MissingQ[p], Return[{}]];
  slot = KeyTake[p, {"Validator", "Round"}]; r = p["Round"]; q = dag["Quorum"];
  certs = Table[With[{parents = Select[DeleteMissing[blockByID[blocks, #] & /@ b["Parents"]], #["Round"] == r + 1 &]},
    With[{support = Select[parents, With[{sp = SupportedProposal[dag, #, slot]}, !MissingQ[sp] && sp["ID"] === p["ID"]] &]},
      If[Length[authorityIDs[support]] >= q, <|"CertificateBlockID" -> b["ID"], "CertificateAuthority" -> b["Validator"],
        "SupporterAuthorities" -> authorityIDs[support], "SupporterBlockIDs" -> Lookup[support, "ID"]|>, Nothing]]],
    {b, Select[blocks, #["Round"] == r + 2 &]}]; certs];

DirectSlotDecision[dag_Association, slot_] := Module[{blocks = dag["Blocks"], q = dag["Quorum"], ss, r, proposals, supporters,
    supportByProposal, certificates, commit, observedR1, nonSupport, skipOK, skipEvidence},
  ss = slotSpec[slot, blocks]; If[MissingQ[ss], Return[<|"Decision" -> "Undecided", "Reason" -> "slot not found", "Threshold" -> q|>]];
  r = ss["Round"]; proposals = SortBy[Select[blocks, #["Validator"] === ss["Validator"] && #["Round"] === r &], blockOrder];
  observedR1 = Select[blocks, #["Round"] === r + 1 &];
  supporters[p_] := Select[observedR1, With[{sp = SupportedProposal[dag, #, ss]}, !MissingQ[sp] && sp["ID"] === p["ID"]] &];
  supportByProposal = Association@Table[p["ID"] -> authorityIDs[supporters[p]], {p, proposals}];
  certificates = Association@Table[p["ID"] -> CertificateEvidence[dag, p], {p, proposals}];
  commit = SelectFirst[proposals, Length[DeleteDuplicates[Lookup[Lookup[certificates, #["ID"], {}], "CertificateAuthority"]]] >= q &, Missing["None"]];
  nonSupport[p_] := authorityIDs[Select[observedR1, With[{sp = SupportedProposal[dag, #, ss]}, MissingQ[sp] || sp["ID"] =!= p["ID"]] &]];
  skipOK = If[proposals === {}, Length[authorityIDs[observedR1]] >= q, AllTrue[proposals, Length[nonSupport[#]] >= q &]];
  skipEvidence = If[proposals === {}, <|"NoProposal" -> authorityIDs[observedR1]|>, Association@Table[p["ID"] -> nonSupport[p], {p, proposals}]];
  Which[!MissingQ[commit], <|"Decision" -> "Commit", "Pattern" -> "q distinct r+2 certificate authorities", "Slot" -> ss,
      "Proposal" -> commit["ID"], "SupporterAuthorities" -> supportByProposal[commit["ID"]],
      "CertificateAuthorities" -> DeleteDuplicates[Lookup[certificates[commit["ID"]], "CertificateAuthority"]],
      "CertificateIDs" -> Lookup[certificates[commit["ID"]], "CertificateBlockID"], "SkipAuthorities" -> <||>, "Threshold" -> q|>,
    skipOK, <|"Decision" -> "Skip", "Pattern" -> "conservative q distinct r+1 non-support authorities", "Slot" -> ss,
      "Proposal" -> Missing["NoCommittedProposal"], "SupporterAuthorities" -> supportByProposal, "CertificateAuthorities" -> {},
      "CertificateIDs" -> {}, "SkipAuthorities" -> skipEvidence, "Threshold" -> q|>,
    True, <|"Decision" -> "Undecided", "Pattern" -> "insufficient direct evidence", "Slot" -> ss,
      "Proposal" -> Missing["None"], "SupporterAuthorities" -> supportByProposal, "CertificateAuthorities" -> certificates,
      "CertificateIDs" -> Flatten[Lookup[Flatten[Values[certificates]], "CertificateBlockID", {}]], "SkipAuthorities" -> skipEvidence, "Threshold" -> q|>]];

SimulateMysticeti[p_Association] := Module[{x = Join[defaults, p], dag, slots, decisions, samples, lat, committed, total, honest},
  SeedRandom[x["Seed"]]; dag = GenerateMysticetiDAG[x];
  slots = Select[dag["Blocks"], #["Validator"] == 1 && #["Round"] <= x["Rounds"] - 2 && !TrueQ[#["Equivocation"]] &];
  decisions = DirectSlotDecision[dag, KeyTake[#, {"Validator", "Round"}]] & /@ slots; committed = Count[Lookup[decisions, "Decision"], "Commit"]; total = Length[decisions];
  samples = Max[250, x["SyntheticSamples"]];
  lat = Table[Max[1., 3 x["BaseDelayMS"] + Total[RandomVariate[NormalDistribution[0, x["JitterMS"]], 3]] +
    45 x["CrashCount"] + 25 x["ByzantineCount"]], {samples}];
  honest = Select[dag["Blocks"], #["Status"] === "Honest" &];
  <|"Parameters" -> x, "DAG" -> dag, "Decisions" -> decisions, "SyntheticSamples" -> samples,
    "CommitLatenciesMS" -> N[lat], "P50Milliseconds" -> Round[Median[lat]], "P95Milliseconds" -> Round[Quantile[lat, .95]],
    "CommittedRate" -> If[total == 0, 0., N[committed/total]], "SkippedRate" -> If[total == 0, 0., N[Count[Lookup[decisions, "Decision"], "Skip"]/total]],
    "UndecidedRate" -> If[total == 0, 0., N[Count[Lookup[decisions, "Decision"], "Undecided"]/total]],
    "InvariantChecks" -> <|"ParentReferencesExist" -> dag["ValidParentReferences"], "DistinctParentAuthorities" -> dag["DistinctParentAuthorities"],
      "HonestDistinctParentAuthorities" -> AllTrue[honest, DuplicateFreeQ[Lookup[DeleteMissing[blockByID[dag["Blocks"], #] & /@ #["Parents"]], "Validator"]] &]|>,
    "ScientificLabel" -> "uncalibrated synthetic sensitivity model",
    "TimingAssumptions" -> "500 seeded samples by default; latency=max(1,3*base delay + sum of three Gaussian message-delay perturbations + 45 ms/crash + 25 ms/Byzantine). Not a WAN simulation or production prediction."|>];

RunFaultSweep[] := Flatten[Table[With[{s = SimulateMysticeti[<|"F" -> 3, "Rounds" -> 9, "CrashCount" -> c, "JitterMS" -> j, "Seed" -> 42|>]},
  <|"CrashCount" -> c, "JitterMS" -> j, "P50Milliseconds" -> s["P50Milliseconds"], "P95Milliseconds" -> s["P95Milliseconds"],
    "CommittedRate" -> s["CommittedRate"], "Model" -> "uncalibrated synthetic sensitivity model"|>], {c, 0, 3}, {j, {10., 40., 80., 140.}}], 1];

DAGVisualization[dag_Association] := Module[{b = dag["Blocks"], colors, labels, coords},
  colors = <|"Honest" -> RGBColor[.1, .75, .95], "Byzantine" -> RGBColor[1, .35, .3], "Equivocation" -> RGBColor[1, .65, .15]|>;
  labels = Association@Table[x["ID"] -> ("V" <> ToString[x["Validator"]] <> " / R" <> ToString[x["Round"]]), {x, b}];
  coords = Association@Table[x["ID"] -> {x["Round"], -x["Validator"] + If[x["Equivocation"], .25, 0]}, {x, b}];
  Graph[Lookup[b, "ID"], dag["Edges"], VertexCoordinates -> coords, VertexLabels -> Placed[labels, Tooltip],
    VertexStyle -> Association@Table[x["ID"] -> Lookup[colors, x["Status"], Gray], {x, b}], EdgeStyle -> Directive[GrayLevel[.55], Opacity[.55]],
    Background -> RGBColor[.035, .05, .09], ImageSize -> 1100, PlotLabel -> Style["Mysticeti educational round DAG (equivocations do not add authority)", 18, White]]];
SafetyEnvelopePlot[] := Module[{d = Select[SafetyEnvelopeData[45], MemberQ[{0, 1, 2, 3, 5, 10, 15}, #["Byzantine"]] &], groups}, groups = GroupBy[d, #["Byzantine"] &];
  ListLinePlot[(Map[{#["CommitteeSize"], #["HonestIntersection"]} &, #] & /@ Values[groups]), PlotLegends -> ("b=" <> ToString[#] & /@ Keys[groups]),
    Frame -> True, FrameLabel -> {"committee size n", "honest intersection lower bound"}, PlotTheme -> "Detailed", ImageSize -> 900,
    PlotLabel -> "Equal-vote safety envelope: q=floor(2n/3)+1; margin=max(0,2q-n-b)"]];
FaultFrontierPlot[] := Module[{d = RunFaultSweep[]}, ListLinePlot[Table[Cases[d, a_ /; a["JitterMS"] == j :> {a["CrashCount"], a["P95Milliseconds"]}], {j, {10., 40., 80., 140.}}],
  PlotLegends -> (ToString[#] <> " ms jitter" & /@ {10, 40, 80, 140}), Frame -> True, FrameLabel -> {"crashed validators", "synthetic P95 (ms)"}, ImageSize -> 900,
  PlotLabel -> "Uncalibrated synthetic sensitivity model (three message delays; not a prediction)"]];
productionRows[] := Module[{raw = Import[FileNameJoin[{packageRoot, "data", "paper_production_benchmark.csv"}], "CSV"]},
  AssociationThread[First[raw], #] & /@ Rest[raw]];
ProductionComparisonPlot[] := Module[{d = productionRows[]}, BarChart[Transpose[Lookup[d, {"P50Milliseconds", "P95Milliseconds"}]], ChartLayout -> "Grouped",
  ChartLegends -> Lookup[d, "Protocol"], ChartLabels -> {Placed[{"P50", "P95"}, Axis], None}, Frame -> True, FrameLabel -> {None, "latency (ms)"}, ImageSize -> 900,
  PlotLabel -> "CSV fixture/transcription: Mysticeti NDSS 2025 Table I, committee 137, load 5000 TPS"]];

ImportRustTrace[path_String] := Import[path, "RawJSON"];
rustKeys[a_, keys_] := AssociationQ[a] && Sort[Keys[a]] === Sort[keys];
rustDistinctQ[x_] := ListQ[x] && DuplicateFreeQ[x];
rustStake[authors_, stakes_] := Total[Lookup[stakes, DeleteDuplicates[authors], Missing["UnknownAuthor"]]];

ValidateRustTrace[path_String] := ValidateRustTrace[ImportRustTrace[path]];
ValidateRustTrace[t_Association] := Module[{config, committee, blocks, events, decisions, invariants, authorityIDs, blockIDs,
    stakes, total, quorum, slots, checks, decisionSlots, commitDecisions},
  config = Lookup[t, "config", <||>]; committee = Lookup[t, "committee", {}]; blocks = Lookup[t, "blocks", {}];
  events = Lookup[t, "events", {}]; decisions = Lookup[t, "decisions", {}]; invariants = Lookup[t, "invariants", <||>];
  authorityIDs = Lookup[committee, "id", {}]; blockIDs = Lookup[blocks, "id", {}]; stakes = AssociationThread[authorityIDs, Lookup[committee, "stake", {}]];
  total = Total[Values[stakes]]; quorum = Floor[2 total/3] + 1; slots = Lookup[config, "slots", 0];
  decisionSlots = Lookup[decisions, "slot", {}]; commitDecisions = Select[decisions, Lookup[#, "kind", ""] === "commit" &];
  checks = <|
    "Schema" -> (Lookup[t, "schema_version", ""] === "mysticeti-twin.trace.v1" &&
      rustKeys[t, {"schema_version", "scope", "config", "seed", "committee", "blocks", "events", "decisions", "invariants", "evidence_labels"}] &&
      rustKeys[config, {"seed", "stakes", "slots", "latency_min_ms", "latency_max_ms", "round_duration_ms", "packet_loss", "crash_authorities", "byzantine_authorities"}] &&
      AllTrue[committee, rustKeys[#, {"id", "stake"}] &] && AllTrue[blocks, rustKeys[#, {"id", "author", "round", "slot", "variant", "parents"}] &] &&
      AllTrue[events, rustKeys[#, {"sequence", "scheduled_at_ms", "outcome_at_ms", "status", "block_id", "sender", "receiver"}] &] &&
      AllTrue[decisions, rustKeys[#, {"slot", "proposal_id", "proposal_round", "kind", "support_stake", "certificate_author_stake", "support_authors", "certificate_authors", "evidence"}] &] &&
      rustKeys[invariants, {"assumptions", "checks", "all_passed"}] && AllTrue[Lookup[invariants, "assumptions", {}], rustKeys[#, {"name", "satisfied", "detail"}] &] &&
      AllTrue[Lookup[invariants, "checks", {}], rustKeys[#, {"name", "passed", "detail"}] &]),
    "ConfigStakesMatchCommittee" -> (Lookup[config, "stakes", {}] === Lookup[committee, "stake", {}] &&
      Lookup[t, "seed", Missing[]] === Lookup[config, "seed", Missing[]] && authorityIDs === Table["authority-" <> ToString[i], {i, 0, Length[committee] - 1}]),
    "TotalAndQuorumMath" -> (total > 0 && quorum === Quotient[2 total, 3] + 1 &&
      AllTrue[Cases[Lookup[invariants, "checks", {}], a_ /; Lookup[a, "name", ""] === "commit_has_certificate_author_quorum"],
        StringContainsQ[Lookup[#, "detail", ""], "threshold=" <> ToString[quorum]] &]),
    "BlockReferences" -> (rustDistinctQ[authorityIDs] && rustDistinctQ[blockIDs] && AllTrue[blocks,
      MemberQ[authorityIDs, Lookup[#, "author", Missing[]]] && AllTrue[Lookup[#, "parents", {}], MemberQ[blockIDs, #] &] &]),
    "EventReferences" -> AllTrue[events, Function[e, MemberQ[authorityIDs, Lookup[e, "sender", Missing[]]] &&
      MemberQ[authorityIDs, Lookup[e, "receiver", Missing[]]] && MemberQ[blockIDs, Lookup[e, "block_id", Missing[]]] &&
      Lookup[FirstCase[blocks, b_ /; Lookup[b, "id", None] === Lookup[e, "block_id", None], <||>], "author", None] === Lookup[e, "sender", Missing[]]]],
    "ReportedAssumptionsSatisfied" -> (Lookup[invariants, "assumptions", {}] =!= {} && AllTrue[Lookup[invariants, "assumptions", {}], TrueQ[Lookup[#, "satisfied", False]] &]),
    "ReportedChecksPassed" -> (Lookup[invariants, "checks", {}] =!= {} && AllTrue[Lookup[invariants, "checks", {}], TrueQ[Lookup[#, "passed", False]] &] && TrueQ[Lookup[invariants, "all_passed", False]]),
    "DecisionAuthorsDistinct" -> AllTrue[decisions, rustDistinctQ[Lookup[#, "support_authors", {}]] && rustDistinctQ[Lookup[#, "certificate_authors", {}]] &],
    "DecisionStakesRecompute" -> AllTrue[decisions, With[{sa = Lookup[#, "support_authors", {}], ca = Lookup[#, "certificate_authors", {}]},
      SubsetQ[authorityIDs, sa] && SubsetQ[authorityIDs, ca] && rustStake[sa, stakes] === Lookup[#, "support_stake", Missing[]] &&
        rustStake[ca, stakes] === Lookup[#, "certificate_author_stake", Missing[]]] &],
    "CommitsHaveCertificateAuthorQuorum" -> AllTrue[commitDecisions, Lookup[#, "certificate_author_stake", 0] >= quorum &],
    "EveryConfiguredSlotDecided" -> (IntegerQ[slots] && slots > 0 && Sort[DeleteDuplicates[decisionSlots]] === Range[0, slots - 1])|>;
  <|"Passed" -> Count[Values[checks], True], "Failed" -> Count[Values[checks], Except[True]], "AllPassed" -> And @@ Values[checks],
    "Checks" -> checks, "Scope" -> "Independent Mathematica conformance audit of recorded Rust schema, references, arithmetic, and evidence; not a reimplementation or proof of the Rust DAG."|>];

RustTraceSummary[path_String] := RustTraceSummary[ImportRustTrace[path]];
RustTraceSummary[t_Association] := Module[{stakes = Lookup[Lookup[t, "committee", {}], "stake", {}], decisions = Lookup[t, "decisions", {}], total},
  total = Total[stakes]; <|"SchemaVersion" -> Lookup[t, "schema_version", Missing[]], "Seed" -> Lookup[t, "seed", Missing[]],
    "Authorities" -> Length[stakes], "TotalStake" -> total, "Quorum" -> Floor[2 total/3] + 1, "Blocks" -> Length[Lookup[t, "blocks", {}]],
    "Events" -> Length[Lookup[t, "events", {}]], "Decisions" -> Length[decisions], "Commits" -> Count[Lookup[decisions, "kind", {}], "commit"]|>];

RustFaultSweepPlot[path_String] := Module[{raw = Import[path, "CSV"], rows, groups, losses, commitCounts, runCounts, slots = 8},
  rows = AssociationThread[First[raw], #] & /@ Rest[raw];
  rows = Map[MapAt[ToExpression, #, {{"seed"}, {"packet_loss"}, {"blocks"}, {"commits"}}] &, rows];
  groups = GroupBy[rows, #["packet_loss"] &]; losses = Sort[Keys[groups]];
  commitCounts = Total[Lookup[groups[#], "commits"]] & /@ losses; runCounts = Length[groups[#]] & /@ losses;
  ListLinePlot[{Transpose[{losses, commitCounts}], Transpose[{losses, commitCounts/(slots runCounts)}]},
    PlotMarkers -> Automatic, PlotLegends -> {"commits (count)", "commit rate (commits / 8 slots / run)"}, Frame -> True,
    FrameLabel -> {"packet loss", "commit evidence"}, PlotRange -> All, ImageSize -> 900,
    PlotLabel -> "Rust event-driven deterministic campaign (actual sweep CSV; 20 seeds)"]];

mkBlock[id_, v_, r_, parents_: {}] := <|"ID" -> id, "Validator" -> v, "Round" -> r, "Parents" -> parents, "Status" -> "Honest", "Crashed" -> False, "Equivocation" -> False|>;
fixtureDag[supportCount_, certCount_, duplicate_: False, nonSupport_: False] := Module[{q = 3, p, r1, certParents, r2, blocks},
  p = mkBlock["p", 1, 1];
  r1 = Table[mkBlock["s" <> ToString[i], i, 2, If[i <= supportCount && !nonSupport, {"p"}, {}]], {i, 1, 4}];
  If[duplicate, r1 = Join[r1, {mkBlock["s1e", 1, 2, {"p"}]}]];
  certParents = Lookup[Select[r1, #["ID"] =!= "s1e" &], "ID"];
  r2 = Table[mkBlock["c" <> ToString[i], i, 3, If[i <= certCount, certParents, {}]], {i, 1, 4}]; blocks = Join[{p}, r1, r2];
  <|"Blocks" -> blocks, "Quorum" -> q, "CommitteeSize" -> 4, "ValidParentReferences" -> True, "DistinctParentAuthorities" -> !duplicate|>];

RunValidationSuite[] := Module[{commitDag, fewCertDag, fewSupportDag, skipDag, equivDag, generated, replay, csv, expected, checks, sp},
  commitDag = fixtureDag[3, 3]; fewCertDag = fixtureDag[3, 2]; fewSupportDag = fixtureDag[2, 3]; skipDag = fixtureDag[0, 0, False, True]; equivDag = fixtureDag[2, 3, True];
  generated = GenerateMysticetiDAG[<|"F" -> 2, "Rounds" -> 6, "Seed" -> 42, "ByzantineCount" -> 1, "Equivocation" -> True|>];
  replay = GenerateMysticetiDAG[<|"F" -> 2, "Rounds" -> 6, "Seed" -> 42, "ByzantineCount" -> 1, "Equivocation" -> True|>];
  csv = productionRows[]; expected = {{"Bullshark", 137, 5000, 2890, 4600}, {"Mysticeti-C", 137, 5000, 650, 975}};
  sp = SupportedProposal[equivDag, "c1", <|"Validator" -> 1, "Round" -> 1|>];
  checks = <|
    "FullSupportAndCertificatesCommit" -> DirectSlotDecision[commitDag, <|"Validator" -> 1, "Round" -> 1|>]["Decision"] === "Commit",
    "FewerThanQCertificateAuthoritiesUndecided" -> DirectSlotDecision[fewCertDag, <|"Validator" -> 1, "Round" -> 1|>]["Decision"] === "Undecided",
    "QMinusOneSupportAuthoritiesUndecided" -> DirectSlotDecision[fewSupportDag, <|"Validator" -> 1, "Round" -> 1|>]["Decision"] === "Undecided",
    "QNonSupportAuthoritiesSkip" -> DirectSlotDecision[skipDag, <|"Validator" -> 1, "Round" -> 1|>]["Decision"] === "Skip",
    "DuplicateAuthorEquivocationDoesNotIncreaseCount" -> DirectSlotDecision[equivDag, <|"Validator" -> 1, "Round" -> 1|>]["Decision"] =!= "Commit",
    "SupportedProposalDeterministicallySelectsOne" -> AssociationQ[sp] && sp["ID"] === "p",
    "DeterministicReplay" -> SameQ[generated, replay], "ParentReferencesExist" -> TrueQ[generated["ValidParentReferences"]],
    "DistinctParentAuthorities" -> TrueQ[generated["DistinctParentAuthorities"]],
    "CSVFixtureTranscriptionConsistency" -> (Lookup[csv, {"Protocol", "CommitteeSize", "LoadTPS", "P50Milliseconds", "P95Milliseconds"}] === expected),
    "CSVFixtureRatioConsistency" -> Abs[N[csv[[1, "P50Milliseconds"]]/csv[[2, "P50Milliseconds"]]] - 4.446153846] < 10^-6 && Abs[N[csv[[1, "P95Milliseconds"]]/csv[[2, "P95Milliseconds"]]] - 4.717948718] < 10^-6,
    "SyntheticSampleFloor" -> SimulateMysticeti[<|"F" -> 1, "Rounds" -> 4, "SyntheticSamples" -> 10|>]["SyntheticSamples"] >= 250|>;
  <|"Passed" -> Count[Values[checks], True], "Failed" -> Count[Values[checks], False], "AllPassed" -> And @@ Values[checks], "Checks" -> checks,
    "Scope" -> "Educational equal-authority fixtures and CSV transcription consistency; not independent paper verification or complete protocol/Lean proof."|>];
End[]; EndPackage[];
