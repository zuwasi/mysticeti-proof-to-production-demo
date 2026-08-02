#!/usr/bin/env wolframscript
projectDir = DirectoryName[$InputFileName]; SetDirectory[projectDir]; If[!DirectoryQ["exports"], CreateDirectory["exports"]];
Get[FileNameJoin[{projectDir, "src", "MysticetiModel.wl"}]]; SeedRandom[42];
rustTracePath = FileNameJoin[{projectDir, "exports", "rust_demo_trace.json"}]; rustSweepPath = FileNameJoin[{projectDir, "exports", "rust_fault_sweep.csv"}];
If[AnyTrue[{rustTracePath, rustSweepPath}, !FileExistsQ[#] || FileByteCount[#] == 0 &], Print["BUILD FAILED: required Rust evidence is missing or empty"]; Exit[1]];
validation = RunValidationSuite[]; rustValidation = ValidateRustTrace[rustTracePath]; rustSummary = RustTraceSummary[rustTracePath]; sweep = RunFaultSweep[]; dag = GenerateMysticetiDAG[<|"F" -> 2, "Rounds" -> 7, "Seed" -> 42, "ByzantineCount" -> 1, "Equivocation" -> True|>];
rustSweep = Import[rustSweepPath, "CSV"]; rustSweepValid = First[rustSweep] === {"seed", "packet_loss", "blocks", "commits", "all_invariants_passed"} && Length[Rest[rustSweep]] === 80 &&
  Length[DeleteDuplicates[Rest[rustSweep][[All, 1]]]] === 20 && AllTrue[Rest[rustSweep][[All, 5]], ToLowerCase[ToString[#]] === "true" &];
Export["exports/dag.png", DAGVisualization[dag], ImageResolution -> 150]; Export["exports/safety_envelope.png", SafetyEnvelopePlot[], ImageResolution -> 150];
Export["exports/fault_frontier.png", FaultFrontierPlot[], ImageResolution -> 150]; Export["exports/production_comparison.png", ProductionComparisonPlot[], ImageResolution -> 150];
Export["exports/rust_fault_sweep.png", RustFaultSweepPlot[rustSweepPath], ImageResolution -> 150];
Export["exports/fault_sweep.csv", Prepend[(Values[#] & /@ sweep), Keys[First[sweep]]], "CSV"];
Export["exports/validation_results.json", validation, "JSON"];
report = "# Mysticeti Digital Twin Validation\n\n- Wolfram fixture checks: " <> ToString[validation["Passed"]] <> "/" <> ToString[validation["Passed"] + validation["Failed"]] <> " passed\n- Rust cross-language conformance checks: " <> ToString[rustValidation["Passed"]] <> "/" <> ToString[rustValidation["Passed"] + rustValidation["Failed"]] <> " passed\n- Rust trace summary: " <> ToString[rustSummary, InputForm] <> "\n- All passed: " <> ToString[TrueQ[validation["AllPassed"]] && TrueQ[rustValidation["AllPassed"]]] <> "\n\nThe Rust check is an independent schema/reference/stake/evidence conformance audit, not a reimplementation or formal proof of the Rust DAG. Timing exports from Wolfram remain uncalibrated synthetic sensitivity; the Rust sweep is actual output from deterministic event-driven campaigns, not a production Sui benchmark. Lean is not invoked by this lane.\n";
Export["exports/validation_report.md", report, "Text"];
auditCode = RunProcess[{"wolframscript", "-file", FileNameJoin[{projectDir, "proof_audit.wls"}]}]; If[auditCode["ExitCode"] =!= 0, Print[auditCode["StandardOutput"]]; Print[auditCode["StandardError"]]; Exit[1]];
required = {"dag.png", "safety_envelope.png", "fault_frontier.png", "production_comparison.png", "rust_fault_sweep.png", "fault_sweep.csv", "rust_demo_trace.json", "rust_fault_sweep.csv", "validation_results.json", "audit_report.txt", "validation_report.md"};
bad = Select[required, !FileExistsQ[FileNameJoin[{projectDir, "exports", #}]] || FileByteCount[FileNameJoin[{projectDir, "exports", #}]] == 0 &];
If[bad =!= {} || !TrueQ[validation["AllPassed"]] || !TrueQ[rustValidation["AllPassed"]] || !TrueQ[rustSweepValid], Print["BUILD FAILED: ", bad]; Exit[1]]; Print["BUILD OK: ", Length[required], " non-empty exports; Wolfram validation ", validation["Passed"], "/", validation["Passed"] + validation["Failed"], "; Rust conformance ", rustValidation["Passed"], "/", rustValidation["Passed"] + rustValidation["Failed"], "; Rust sweep 80/80 rows valid"];
