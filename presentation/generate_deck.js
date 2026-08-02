const path = require('path');
const pptxgen = require('pptxgenjs');

const pptx = new pptxgen();
pptx.layout = 'LAYOUT_WIDE';
pptx.author = 'Daniel Liezrowice / ESL, generated with Amp';
pptx.company = 'ESL';
pptx.subject = 'Scientifically bounded Mysticeti consensus digital twin workflow';
pptx.title = 'Mysticeti Consensus Digital Twin — Amp + Wolfram Mathematica + Lean 4 + Rust';
pptx.lang = 'en-US';
pptx.theme = {
  headFontFace: 'Aptos Display', bodyFontFace: 'Aptos', lang: 'en-US'
};
pptx.defineSlideMaster({
  title: 'DARK',
  background: { color: '07111F' },
  objects: [
    { rect: { x: 0, y: 0, w: 13.333, h: 0.09, fill: { color: '24D5FF' }, line: { color: '24D5FF' } } },
  ],
  slideNumber: { x: 12.45, y: 7.13, w: 0.35, h: 0.18, color: '8FA3BC', fontSize: 9, align: 'right', margin: 0 }
});

const C = { bg:'07111F', bg2:'0B172A', card:'10223A', card2:'132943', cyan:'24D5FF', violet:'8B5CF6', green:'2DD4A7', amber:'FFB547', white:'FFFFFF', text:'D9E7F5', muted:'8FA3BC', red:'FF6B6B', line:'25425F', black:'02060D' };
const ROOT = path.resolve(__dirname, '..');
const AS = {
  logo: path.join(__dirname, 'assets', 'ESL_Logo.png'),
  daniel: path.join(__dirname, 'assets', 'daniel-liezrowice.jpg'),
  dag: path.join(ROOT, 'exports', 'dag.png'), safety: path.join(ROOT, 'exports', 'safety_envelope.png'),
  fault: path.join(ROOT, 'exports', 'fault_frontier.png'), prod: path.join(ROOT, 'exports', 'production_comparison.png'),
  rustFault: path.join(ROOT, 'exports', 'rust_fault_sweep.png'), terminal: path.join(__dirname, 'assets', 'rust_terminal.png'),
  wslTerminal: path.join(__dirname, 'assets', 'wsl_repro_terminal.png')
};
const ESL='https://eswlab.com/', LINKEDIN='https://il.linkedin.com/in/liezrowice';
const REPO='https://github.com/zuwasi/mysticeti-proof-to-production-demo';
const OUT=path.join(__dirname, 'Mysticeti_Consensus_Digital_Twin_ESL.pptx');

function addText(s, text, x,y,w,h, o={}) {
  s.addText(text,{x,y,w,h,fontFace:o.fontFace||'Aptos',fontSize:o.fontSize||16,color:o.color||C.text,
    bold:o.bold||false,margin:o.margin===undefined?0:o.margin,breakLine:false,fit:'shrink',valign:o.valign||'mid',
    align:o.align||'left',italic:o.italic||false,bullet:o.bullet,paraSpaceAfterPt:o.paraSpaceAfterPt||0,
    isTextBox:true,hyperlink:o.hyperlink,transparency:o.transparency||0});
}
function rect(s,x,y,w,h,fill=C.card,r=0.12,line=C.line) {
  s.addShape(r?pptx.ShapeType.roundRect:pptx.ShapeType.rect,{x,y,w,h,rectRadius:r,fill:{color:fill},line:{color:line,width:0.8}});
}
function line(s,x,y,w,h,color=C.line,width=1,dash='solid') { s.addShape(pptx.ShapeType.line,{x,y,w,h,line:{color,width,dashType:dash,beginArrowType:'none',endArrowType:'none'}}); }
function title(s,kicker,head,sub='') {
  addText(s,kicker.toUpperCase(),0.58,0.28,8.5,0.24,{fontSize:10,bold:true,color:C.cyan});
  addText(s,head,0.58,0.58,12.0,0.6,{fontSize:35,bold:true,color:C.white});
  if(sub) addText(s,sub,0.6,1.18,11.8,0.35,{fontSize:15,color:C.muted});
}
function tag(s,text,x,y,color=C.green,w=1.55) {
  s.addShape(pptx.ShapeType.roundRect,{x,y,w,h:0.27,rectRadius:0.08,fill:{color,transparency:80},line:{color,width:0.8}});
  addText(s,text,x+0.08,y+0.015,w-0.16,0.22,{fontSize:8.5,bold:true,color,align:'center'});
}
function footer(s,evidence='') {
  line(s,0.58,7.06,12.15,0,C.line,0.7);
  addText(s,[{text:ESL,options:{hyperlink:{url:ESL},color:C.cyan,underline:true}}],0.6,7.12,2.0,0.18,{fontSize:9,color:C.cyan});
  addText(s,[{text:'GitHub: mysticeti-proof-to-production-demo',options:{hyperlink:{url:REPO},color:C.muted,underline:true}}],2.55,7.12,4.25,0.18,{fontSize:8.5,color:C.muted});
  if(evidence) addText(s,evidence.toUpperCase(),9.35,7.12,2.85,0.18,{fontSize:8.5,bold:true,color:C.muted,align:'right'});
}
function logo(s,x=11.52,y=0.25,w=1.18,h=0.48) {
  s.addShape(pptx.ShapeType.roundRect,{x,y,w,h,rectRadius:0.08,fill:{color:'FFFFFF'},line:{color:'D6E2EF',width:0.7},hyperlink:{url:ESL}});
  s.addImage({path:AS.logo,...contain(AS.logo,x+0.09,y+0.08,w-0.18,h-0.16),hyperlink:{url:ESL}});
}
const dims = new Map([[AS.logo,[639,200]],[AS.daniel,[1179,1179]],[AS.dag,[2292,2420]],[AS.safety,[2023,1213]],[AS.fault,[2111,1206]],[AS.prod,[2077,1169]],[AS.rustFault,[1800,1000]],[AS.terminal,[1800,980]],[AS.wslTerminal,[1800,980]]]);
function contain(path,x,y,w,h){
  const [iw,ih]=dims.get(path), scale=Math.min(w/iw,h/ih), nw=iw*scale, nh=ih*scale;
  return {x:x+(w-nw)/2,y:y+(h-nh)/2,w:nw,h:nh};
}
function base(kicker,head,sub='',evidence='') { const s=pptx.addSlide('DARK'); title(s,kicker,head,sub); logo(s); footer(s,evidence); return s; }
function dot(s,x,y,color=C.cyan,r=0.10){ s.addShape(pptx.ShapeType.ellipse,{x,y,w:r,h:r,fill:{color},line:{color}}); }
function cardText(s,x,y,w,h,label,body,color=C.cyan){ rect(s,x,y,w,h); addText(s,label.toUpperCase(),x+0.2,y+0.15,w-0.4,0.25,{fontSize:10,bold:true,color}); addText(s,body,x+0.2,y+0.48,w-0.4,h-0.62,{fontSize:15,color:C.text,valign:'top'}); }

// 1 — Hero
{
 const s=pptx.addSlide('DARK');
 s.addShape(pptx.ShapeType.ellipse,{x:8.8,y:-1.2,w:5.4,h:5.4,fill:{color:C.violet,transparency:80},line:{color:C.violet,transparency:100}});
 s.addShape(pptx.ShapeType.ellipse,{x:9.8,y:2.3,w:3.7,h:3.7,fill:{color:C.cyan,transparency:87},line:{color:C.cyan,transparency:100}});
 logo(s,11.35,0.35,1.35,0.55);
 tag(s,'PRODUCTION CONTEXT',0.65,0.55,C.green,1.7);
 addText(s,'Mysticeti Consensus\nDigital Twin',0.65,1.22,7.9,1.55,{fontSize:43,bold:true,color:C.white,valign:'top'});
 addText(s,'Amp + Wolfram Mathematica + Lean 4 + Rust',0.68,2.98,7.8,0.5,{fontSize:23,bold:true,color:C.cyan});
 addText(s,'An executable + formally bounded microscope for the quorum and direct-decision core behind production Sui Layer-1 consensus.',0.68,3.62,7.35,0.92,{fontSize:18,color:C.text,valign:'top'});
 const nodes=[[9.0,1.15,C.cyan],[10.3,0.95,C.green],[11.55,1.55,C.amber],[9.5,2.35,C.violet],[10.8,2.6,C.cyan],[12.05,3.0,C.green],[9.2,3.75,C.amber],[10.6,4.1,C.violet],[11.85,4.55,C.cyan]];
 [[0,1],[1,2],[0,3],[1,4],[2,5],[3,4],[4,5],[3,6],[4,7],[5,8],[6,7],[7,8]].forEach(([a,b])=>line(s,nodes[a][0]+.13,nodes[a][1]+.13,nodes[b][0]-nodes[a][0],nodes[b][1]-nodes[a][1],C.line,1.5));
 nodes.forEach((n,i)=>{s.addShape(pptx.ShapeType.ellipse,{x:n[0],y:n[1],w:.28,h:.28,fill:{color:n[2]},line:{color:C.white,width:i%3===0?1.2:.4}})});
 rect(s,0.67,5.25,7.45,0.92,C.bg2,0.12,C.line);
 addText(s,'Scientific boundary',0.9,5.43,1.4,0.25,{fontSize:11,bold:true,color:C.amber});
 addText(s,'Bounded Rust research twin · not production equivalence · not a production benchmark reproduction',2.28,5.35,5.55,0.46,{fontSize:13.5,color:C.text});
 footer(s,'EXECUTABLE + FORMAL EVIDENCE');
}

// 2
{
 const s=base('01 · production relevance','Why Mysticeti matters','A research result with a real deployment path — and an unusually sharp latency target.','PAPER + PUBLIC SOURCES');
 line(s,1.05,3.28,11.1,0,C.line,2);
 const events=[
  {x:1.0,yr:'Oct 2023',t:'Preprint',d:'“Reaching the Limits of Latency with Uncertified DAGs”',c:C.violet},
  {x:4.0,yr:'25 Jul 2024',t:'Sui Mainnet',d:'Validators switched to Mysticeti-C consensus.',c:C.green},
  {x:7.25,yr:'NDSS 2025',t:'Peer-reviewed',d:'Mysticeti appears in the NDSS 2025 program.',c:C.cyan},
  {x:10.25,yr:'Goal',t:'3 message rounds',d:'Direct commit path targets three message rounds.',c:C.amber}
 ];
 events.forEach((e,i)=>{dot(s,e.x,3.18,e.c,.22); addText(s,e.yr,e.x-.18,2.25,2.25,.3,{fontSize:12,bold:true,color:e.c}); addText(s,e.t,e.x-.18,2.6,2.25,.38,{fontSize:19,bold:true,color:C.white}); addText(s,e.d,e.x-.18,3.57,2.25,.85,{fontSize:13.5,color:C.text,valign:'top'});});
 rect(s,.72,5.15,11.9,1.15,C.bg2,.12,C.line);
 addText(s,'SOURCE TRAIL',.95,5.35,1.2,.22,{fontSize:10,bold:true,color:C.cyan});
 addText(s,[
  {text:'arxiv.org/abs/2310.14821',options:{hyperlink:{url:'https://arxiv.org/abs/2310.14821'},color:C.cyan,underline:true}},
  {text:'   ·   ',options:{color:C.muted}},
  {text:'ndss-symposium.org/ndss-paper/mysticeti',options:{hyperlink:{url:'https://www.ndss-symposium.org/wp-content/uploads/2025-929-paper.pdf'},color:C.cyan,underline:true}},
  {text:'   ·   ',options:{color:C.muted}},
  {text:'docs.sui.io/.../consensus',options:{hyperlink:{url:'https://docs.sui.io/develop/sui-architecture/consensus'},color:C.cyan,underline:true}}
 ],2.2,5.28,9.9,.4,{fontSize:12.5});
 addText(s,'Production relevance raises the assurance bar: every claim needs a named evidence lane.',.95,5.85,10.9,.28,{fontSize:15,bold:true,color:C.white});
}

// 3
{
 const s=base('02 · epistemic hygiene','One protocol. Five different kinds of claim.','The challenge is not generating output — it is preventing evidence from silently changing meaning.','EVIDENCE GATE');
 const items=[
  ['PAPER CLAIM','What the authors state','citation + normalized claim',C.violet],
  ['RUST DIGITAL TWIN','What bounded traces execute','event-driven replay + audit',C.cyan],
  ['FORMAL PROOF','What follows from assumptions','Lean kernel-checked theorem',C.green],
  ['REPORTED DATA','What Table I reports','local CSV transcription',C.amber],
  ['PRODUCTION SUI','What deployed Sui does','public sources; parity not claimed',C.red]
 ];
 items.forEach((a,i)=>{const y=1.75+i*.92; rect(s,.72,y,11.85,.72,i===4?'241B2A':C.bg2,.1,i===4?C.red:C.line); tag(s,a[0],.92,y+.21,a[3],1.48); addText(s,a[1],2.62,y+.12,3.15,.25,{fontSize:15,bold:true,color:C.white}); addText(s,a[2],6.05,y+.12,4.65,.25,{fontSize:14,color:C.text}); addText(s,i===4?'OUT OF SCOPE':'TRACEABLE',10.9,y+.16,1.28,.22,{fontSize:9,bold:true,color:a[3],align:'right'});});
 addText(s,'No upward inference without a bridge.',8.7,6.5,3.55,.28,{fontSize:15,bold:true,color:C.amber,align:'right'});
}

// 4
{
 const s=base('03 · orchestration','Amp coordinates three lanes — one evidence gate','Rust executes traces, Mathematica explores and audits, Lean proves bounded mathematics.','TRACEABILITY WORKFLOW');
 const boxes=[
  {x:.65,y:2.3,w:1.35,h:1.0,t:'PAPER',d:'claim + source',c:C.violet},
  {x:2.35,y:2.3,w:1.7,h:1.0,t:'AMP LEDGER',d:'scope · status',c:C.cyan},
  {x:4.4,y:2.3,w:1.55,h:1.0,t:'NORMALIZE',d:'schema · n · f · q',c:C.amber},
  {x:6.3,y:1.25,w:2.05,h:.85,t:'RUST',d:'event trace + replay',c:C.amber},
  {x:6.3,y:2.35,w:2.05,h:.85,t:'MATHEMATICA',d:'explore + conform',c:C.cyan},
  {x:6.3,y:3.45,w:2.05,h:.85,t:'LEAN 4',d:'bounded theorem',c:C.green},
  {x:8.8,y:2.3,w:1.8,h:1.0,t:'EVIDENCE GATE',d:'reports + boundaries',c:C.violet},
  {x:11.0,y:2.3,w:1.7,h:1.0,t:'IMPLICATIONS',d:'bounded claims',c:C.amber}
 ];
 boxes.forEach(b=>{rect(s,b.x,b.y,b.w,b.h,C.bg2,.12,b.c); addText(s,b.t,b.x+.12,b.y+.14,b.w-.24,.25,{fontSize:11.5,bold:true,color:b.c,align:'center'}); addText(s,b.d,b.x+.12,b.y+.48,b.w-.24,.22,{fontSize:11.5,color:C.text,align:'center'});});
 [[2,0,3],[4,0,5],[6,0,7]].forEach(([x1,y1,x2])=>{});
 const arrows=[[2,2.35],[4.05,4.4],[5.95,6.3],[8.35,8.8],[10.6,11.0]];
 arrows.forEach(a=>s.addShape(pptx.ShapeType.chevron,{x:a[0],y:2.62,w:a[1]-a[0]-.05,h:.34,fill:{color:C.line},line:{color:C.line}}));
 line(s,6.08,2.72,.22,-1.05,C.line,1.5); line(s,6.08,2.88,.22,1.0,C.line,1.5);
 rect(s,1.05,5.12,11.0,.95,C.card2,.12,C.line);
 addText(s,'EVIDENCE GATE',1.3,5.34,1.55,.25,{fontSize:11,bold:true,color:C.green});
 addText(s,'Every output carries: source → assumption → method → result → limitation.',2.88,5.23,8.7,.45,{fontSize:18,bold:true,color:C.white});
}

// 5
{
 const s=base('04 · trust boundary','Evidence hierarchy — strong does not mean broad','Confidence is attached to the exact statement, not inherited by neighboring protocol claims.','TRUST BOUNDARY');
 const levels=[
  ['FORMALLY PROVED','Lean checks the theorem from explicit finite-set assumptions.',C.green,10.9],
  ['RUST EXECUTABLE','Bounded stake-weighted event traces, strict replay and audit.',C.cyan,9.5],
  ['MATHEMATICA','Independent conformance, fixtures and visualization.',C.violet,8.1],
  ['CSV TRANSCRIPTION','Locally stored paper values — not independent reproduction.',C.amber,6.7],
  ['OUT OF SCOPE','Production parity · full protocol proof · liveness · performance.',C.red,5.3]
 ];
 levels.forEach((a,i)=>{const y=1.7+i*.94,w=a[3]; rect(s,.75,y,w,.68,C.bg2,.08,a[2]); addText(s,a[0],1.0,y+.13,2.15,.22,{fontSize:11,bold:true,color:a[2]}); addText(s,a[1],3.2,y+.10,w-2.65,.3,{fontSize:14,color:C.text});});
 line(s,12.15,1.72,0,4.34,C.line,3); addText(s,'STRONG +\nNARROW',10.98,1.76,1.0,.55,{fontSize:9,bold:true,color:C.green,align:'center'}); addText(s,'NO\nCLAIM',11.08,5.35,.85,.5,{fontSize:9,bold:true,color:C.red,align:'center'});
}

// 6
{
 const s=base('05 · executable fixture','Read the DAG as evidence — not as decoration','Rounds run left-to-right in the source graphic; validator lanes and authority identity govern every count.','EXECUTABLE FIXTURE');
 rect(s,.62,1.62,7.35,5.08,C.black,.12,C.line); s.addImage({path:AS.dag,...contain(AS.dag,.78,1.77,7.02,4.78)});
 const calls=[
  ['ROUNDS','Causal depth, not wall-clock time',C.cyan],['AUTHORITY','One validator = one voting identity',C.green],['EQUIVOCATION','More blocks ≠ more voting power',C.amber],['PARENTS','Distinct-authority references',C.violet]
 ];
 calls.forEach((a,i)=>{const y=1.8+i*1.15; rect(s,8.35,y,4.25,.85,C.bg2,.1,a[2]); addText(s,a[0],8.58,y+.12,1.2,.2,{fontSize:10,bold:true,color:a[2]}); addText(s,a[1],9.74,y+.10,2.55,.42,{fontSize:14.5,bold:true,color:C.white});});
 addText(s,'The generated fixture explicitly checks parent references and distinct parent authorities.',8.48,6.1,3.9,.42,{fontSize:13,color:C.muted});
}

// 7
{
 const s=base('06 · direct-decision anatomy','A certificate is not a commit','The model separates evidence at r, r+1, and r+2 so “I saw support” cannot become accidental finality.','EXECUTABLE FIXTURE');
 const cols=[{x:.65,t:'r',h:'PROPOSAL SLOT',c:C.violet},{x:3.78,t:'r + 1',h:'q SUPPORTERS',c:C.cyan},{x:6.92,t:'r + 2',h:'CERTIFICATE BLOCKS',c:C.green},{x:10.05,t:'DECISION',h:'DIRECT COMMIT',c:C.amber}];
 cols.forEach((a,i)=>{addText(s,a.t,a.x,1.62,2.65,.28,{fontSize:13,bold:true,color:a.c,align:'center'}); rect(s,a.x,2.02,2.65,2.35,C.bg2,.12,a.c); addText(s,a.h,a.x+.16,2.23,2.33,.3,{fontSize:12,bold:true,color:a.c,align:'center'});});
 // proposal
 s.addShape(pptx.ShapeType.hexagon,{x:1.54,y:2.87,w:.85,h:.75,fill:{color:C.violet,transparency:15},line:{color:C.violet,width:2}}); addText(s,'P',1.78,3.04,.36,.22,{fontSize:18,bold:true,color:C.white,align:'center'});
 // supporters
 [0,1,2,3,4].forEach(i=>{dot(s,4.22+(i%3)*.7,2.85+Math.floor(i/3)*.55,C.cyan,.24);}); addText(s,'q distinct\nauthorities',4.21,3.66,1.8,.45,{fontSize:12,bold:true,color:C.white,align:'center'});
 // certificates
 [0,1,2].forEach(i=>{s.addShape(pptx.ShapeType.hexagon,{x:7.38+i*.65,y:2.92,w:.5,h:.46,fill:{color:C.green,transparency:10},line:{color:C.green,width:1.2}})}); addText(s,'each contains q\nsupport authorities',7.27,3.57,1.95,.48,{fontSize:11.5,color:C.text,align:'center'});
 // commit
 s.addShape(pptx.ShapeType.chevron,{x:10.7,y:2.8,w:1.32,h:.82,fill:{color:C.amber},line:{color:C.amber}}); addText(s,'COMMIT',10.82,3.06,1.0,.22,{fontSize:15,bold:true,color:C.bg,align:'center'});
 [3.3,6.43,9.57].forEach(x=>s.addShape(pptx.ShapeType.chevron,{x,y:2.95,w:.38,h:.55,fill:{color:C.line},line:{color:C.line}}));
 rect(s,1.2,5.05,10.95,1.02,'241D16',.12,C.amber);
 addText(s,'DIRECT SKIP',1.48,5.27,1.4,.22,{fontSize:11,bold:true,color:C.amber}); addText(s,'At r + 1, q distinct non-support authorities can justify a conservative direct skip — no r + 2 certificate quorum required.',2.85,5.17,8.8,.46,{fontSize:15,bold:true,color:C.white});
}

// 8
{
 const s=base('07 · interactive experience','Consensus Safety Microscope','A developer changes assumptions, predicts the outcome, then receives inspectable evidence — not a magic score.','EXECUTABLE FIXTURE');
 rect(s,.62,1.62,6.15,4.95,C.black,.12,C.line); s.addImage({path:AS.dag,...contain(AS.dag,.8,1.82,3.75,4.45)});
 rect(s,4.72,1.85,1.78,4.35,C.bg2,.1,C.line);
 ['fault bound f','crashed validators','round count','equivocation','random seed'].forEach((t,i)=>{addText(s,t,4.9,2.05+i*.7,1.35,.2,{fontSize:10.5,bold:true,color:i===3?C.amber:C.text}); line(s,4.92,2.38+i*.7,1.05,0,C.line,3); dot(s,5.45+(i%2)*.25,2.31+i*.7,i===3?C.amber:C.cyan,.16);});
 rect(s,7.1,1.62,5.55,4.95,C.bg2,.12,C.line); addText(s,'RETURNED EVIDENCE',7.38,1.9,2.15,.25,{fontSize:11,bold:true,color:C.cyan});
 const rows=[['Decision','Commit / Skip / Undecided'],['Support','authority IDs at r + 1'],['Certificates','block + authority IDs at r + 2'],['Threshold','explicit q = 2f + 1'],['Invariants','parents valid · authorities distinct']];
 rows.forEach((r,i)=>{const y=2.36+i*.68; line(s,7.38,y+.45,4.75,0,C.line,.6); addText(s,r[0],7.4,y,1.15,.25,{fontSize:12,bold:true,color:i===0?C.green:C.muted}); addText(s,r[1],8.6,y,3.25,.25,{fontSize:14,bold:i===0,color:C.white});});
 tag(s,'PREDICT → VARY → INSPECT',8.35,5.9,C.violet,2.8);
}

// 9
{
 const s=base('08 · Rust architecture','The executable twin is small enough to inspect — complete enough to challenge','Every transition is deterministic; every recorded claim can be replayed and independently audited.','BOUNDED RUST DIGITAL TWIN');
 const parts=[['COMMITTEE','validated, stake-weighted',C.amber],['CANONICAL DAG','transactional insertion',C.cyan],['LOCAL VIEWS','one per authority',C.violet],['EVENT QUEUE','deterministic ordering',C.green],['DECISION ENGINE','direct commit / skip',C.amber],['CAMPAIGN ENGINE','dedicated Rayon pool',C.violet],['STRICT REPLAY','schema + tamper checks',C.cyan],['INVARIANT AUDITOR','independent evidence checks',C.green]];
 parts.forEach((a,i)=>{const x=.68+(i%4)*3.05,y=1.72+Math.floor(i/4)*1.42,w=2.72; rect(s,x,y,w,1.05,C.bg2,.1,a[2]); addText(s,a[0],x+.16,y+.16,w-.32,.22,{fontSize:10.5,bold:true,color:a[2],align:'center'}); addText(s,a[1],x+.16,y+.53,w-.32,.25,{fontSize:13.5,bold:true,color:C.white,align:'center'});});
 rect(s,1.1,5.08,11.05,.9,C.card2,.12,C.line); addText(s,'TRACE CONTRACT',1.35,5.35,1.55,.22,{fontSize:10,bold:true,color:C.cyan}); addText(s,'inputs → receiver-specific events → local DAG transitions → decisions → invariant evidence → strict replay',2.95,5.23,8.72,.42,{fontFace:'Consolas',fontSize:14,bold:true,color:C.white});
 tag(s,'RESEARCH TWIN ≠ PRODUCTION SUI',4.95,6.28,C.red,3.15);
}

// 10
{
 const s=base('09 · causal network','Faults matter because delivery changes what each authority can know','The simulator models causal consequences, not just labels attached after a global DAG is built.','EVENT-DRIVEN TRACE');
 const stages=[['SEND','block created',C.cyan],['DELIVER / DROP','per receiver',C.amber],['LOCAL VIEW','receiver DAG',C.violet],['DEADLINE','round closes',C.red],['PARENTS','selected locally',C.green]];
 stages.forEach((a,i)=>{const x=.58+i*2.55; rect(s,x,1.78,2.12,1.0,C.bg2,.1,a[2]); addText(s,a[0],x+.12,1.96,1.88,.2,{fontSize:10,bold:true,color:a[2],align:'center'}); addText(s,a[1],x+.12,2.31,1.88,.22,{fontSize:13,bold:true,color:C.white,align:'center'}); if(i<4)s.addShape(pptx.ShapeType.chevron,{x:x+2.18,y:2.1,w:.28,h:.4,fill:{color:C.line},line:{color:C.line}});});
 const effects=[['CRASH','stops future sends'],['LOSS','removes receiver-specific delivery'],['EQUIVOCATION','creates competing blocks; stake still deduplicated']];
 effects.forEach((a,i)=>{const x=.75+i*4.08; rect(s,x,3.45,3.7,1.12,'241D16',.1,C.amber); addText(s,a[0],x+.18,3.66,1.05,.2,{fontSize:11,bold:true,color:C.amber}); addText(s,a[1],x+1.18,3.58,2.25,.45,{fontSize:14,bold:true,color:C.white});});
 rect(s,.75,5.08,11.85,.82,'241B2A',.12,C.red); addText(s,'EVIDENCE LIMIT',1.0,5.35,1.45,.2,{fontSize:10,bold:true,color:C.red}); addText(s,'Global campaign aggregates deterministic research scenarios; it is not a calibrated WAN model or Sui production benchmark.',2.5,5.2,9.55,.4,{fontSize:15,bold:true,color:C.white});
}

// 11
{
 const s=base('10 · actual output','The release evidence is executable, not decorative','Actual Windows + Ubuntu WSL reports: tests, clippy, trace replay, deterministic campaigns, and Criterion evidence.','ACTUAL TERMINAL CAPTURE');
 rect(s,.72,1.55,11.9,4.95,C.black,.12,C.line); s.addImage({path:AS.wslTerminal,...contain(AS.wslTerminal,.86,1.68,11.62,4.68)});
 tag(s,'26 TESTS · 0 FAILED / IGNORED',.88,6.48,C.green,2.15); tag(s,'PARALLEL CAMPAIGN TESTS',3.2,6.48,C.cyan,2.05); tag(s,'CLIPPY: WINDOWS + WSL',5.42,6.48,C.green,2.02); tag(s,'WINDOWS [3.5361, 3.5774] s',7.62,6.48,C.amber,2.28); tag(s,'WSL EST. 2.9977 s',10.08,6.48,C.violet,1.62);
}

// 12
{
 const s=base('11 · cross-platform determinism','Same evidence across OS and scheduler','Independent scenario concurrency changes timing and interleaving — never canonical evidence ordering.','VERIFIED REPRODUCIBILITY');
 const runs=[['WINDOWS','jobs = 1',C.cyan],['WINDOWS','jobs = 8',C.cyan],['UBUNTU WSL2','jobs = 1',C.violet],['UBUNTU WSL2','jobs = 8',C.violet]];
 runs.forEach((a,i)=>{const x=.62+i*3.17; rect(s,x,1.65,2.78,.78,C.bg2,.1,a[2]); addText(s,a[0],x+.12,1.78,1.35,.2,{fontSize:10,bold:true,color:a[2]}); addText(s,a[1],x+1.42,1.75,1.12,.25,{fontSize:15,bold:true,color:C.white,align:'right'}); line(s,x+1.39,2.43,0,.42,a[2],1.5);});
 rect(s,.72,2.86,11.85,.86,C.card2,.1,C.green); addText(s,'20-SEED / 80-ROW CAMPAIGN CSV — BYTE IDENTICAL',.98,3.03,4.3,.2,{fontSize:11,bold:true,color:C.green}); addText(s,'5fdc665f5c35cee5c63143860789a9a2a4db831ec65c9e814610d1f6b1764a6a',5.25,2.99,6.95,.29,{fontFace:'Consolas',fontSize:12.5,bold:true,color:C.white});
 rect(s,.72,3.95,11.85,.86,C.card2,.1,C.cyan); addText(s,'DEMO TRACE — WINDOWS = UBUNTU WSL',.98,4.12,4.0,.2,{fontSize:11,bold:true,color:C.cyan}); addText(s,'6272fa854de66bc42512f38d095d7ccf6f75bb85f581dc4acaabf9fbe8ede71d',5.25,4.08,6.95,.29,{fontFace:'Consolas',fontSize:12.5,bold:true,color:C.white});
 const why=[['ORDER','Scheduler interleaving cannot change evidence ordering.'],['PORTABLE','Replay artifact crosses Windows / WSL unchanged.'],['SEMANTICS','Timing changes do not change semantic output.']];
 why.forEach((a,i)=>{const x=.72+i*4.0; rect(s,x,5.08,3.7,.72,C.bg2,.1,[C.green,C.cyan,C.amber][i]); addText(s,a[0],x+.16,5.2,.85,.18,{fontSize:9.5,bold:true,color:[C.green,C.cyan,C.amber][i]}); addText(s,a[1],x+1.02,5.15,2.5,.3,{fontSize:11.5,bold:true,color:C.white});});
 addText(s,'BOUNDARY  Dedicated Rayon pool across independent scenarios · each scenario deterministic · canonical sort before CSV serialization · no global pool mutation.',.76,6.04,11.8,.3,{fontSize:12.5,bold:true,color:C.text});
 addText(s,'WSL CRITERION  32 authorities / 50 slots · [2.9226 s, 2.9977 s estimate, 3.0633 s] · 10 samples · Ubuntu WSL2 · 24 vCPUs',.76,6.35,11.8,.22,{fontSize:10.5,bold:true,color:C.violet});
 addText(s,'NON-CLAIMS  Development-machine WSL evidence; not bare-metal/production; no speedup comparison against Windows; no Ethereum builder flow or Sui production implementation.',.76,6.62,11.8,.24,{fontSize:10.5,bold:true,color:C.red});
}

// 13
{
 const s=base('12 · deterministic campaign','Loss reduces progress in the bounded Rust campaign','20 seeds × 4 loss levels = 80 rows; dedicated Rayon pool across independent deterministic scenarios.','RUST FAULT CAMPAIGN');
 rect(s,.62,1.62,8.25,4.95,'FFFFFF',.12,'D5E1EB'); s.addImage({path:AS.rustFault,...contain(AS.rustFault,.82,1.8,7.85,4.56)});
 rect(s,9.12,1.75,3.48,1.42,C.bg2,.12,C.green); addText(s,'LOSS 0.00',9.4,1.98,2.92,.22,{fontSize:11,bold:true,color:C.green}); addText(s,'8.0 average commits',9.4,2.35,2.92,.32,{fontSize:20,bold:true,color:C.white});
 rect(s,9.12,3.42,3.48,1.42,C.bg2,.12,C.amber); addText(s,'LOSS 0.30',9.4,3.65,2.92,.22,{fontSize:11,bold:true,color:C.amber}); addText(s,'2.7 average commits',9.4,4.02,2.92,.32,{fontSize:20,bold:true,color:C.white});
 rect(s,9.12,5.12,3.48,1.15,'241B2A',.12,C.red); addText(s,'DETERMINISM BOUNDARY',9.4,5.31,2.92,.2,{fontSize:9.5,bold:true,color:C.red}); addText(s,'Canonical sort before CSV; no global pool mutation.',9.4,5.6,2.82,.4,{fontSize:12.5,bold:true,color:C.white});
}

// 13
{
 const s=base('08 · Wolfram lane','A reusable Wolfram architecture — not a notebook monolith','Deterministic package APIs feed notebook exploration, fixtures, plots, CSV evidence, and release reports.','EXECUTABLE FIXTURE');
 const apis=[['GenerateMysticetiDAG','seeded round DAG'],['SupportedProposal','deterministic BFS'],['CertificateEvidence','q supporters / authority'],['DirectSlotDecision','commit · skip · undecided'],['RunValidationSuite','12 named checks']];
 apis.forEach((a,i)=>{const x=.65+(i%3)*2.65,y=1.68+Math.floor(i/3)*1.08; rect(s,x,y,2.35,.78,C.bg2,.1,i===4?C.green:C.line); addText(s,a[0],x+.13,y+.12,2.09,.22,{fontFace:'Consolas',fontSize:11,bold:true,color:i===4?C.green:C.cyan}); addText(s,a[1],x+.13,y+.42,2.09,.18,{fontSize:10.5,color:C.muted});});
 s.addShape(pptx.ShapeType.chevron,{x:8.7,y:2.3,w:.55,h:.65,fill:{color:C.line},line:{color:C.line}});
 rect(s,9.45,1.68,3.2,2.45,C.card2,.12,C.violet); addText(s,'GENERATED EVIDENCE',9.68,1.9,2.7,.25,{fontSize:11,bold:true,color:C.violet});
 addText(s,'dag.png\nsafety_envelope.png\nfault_frontier.png\nvalidation_results.json',9.7,2.35,2.55,1.25,{fontFace:'Consolas',fontSize:13.5,color:C.white,valign:'top'});
 rect(s,.65,4.45,12.0,1.55,C.black,.1,C.line);
 addText(s,'SupportedProposal[dag_, block_, slot_] :=',.9,4.7,5.1,.25,{fontFace:'Consolas',fontSize:14,bold:true,color:C.cyan});
 addText(s,'breadth-first ancestors  →  sort {round, validator, ID}\n→ return at most one proposal deterministically',.9,5.05,5.55,.56,{fontFace:'Consolas',fontSize:13,color:C.text,valign:'top'});
 line(s,6.62,4.67,0,1.08,C.line,1);
 addText(s,'Authority deduplication is inside the evidence path —\nequivocation never manufactures quorum weight.',6.92,4.83,5.1,.64,{fontSize:16,bold:true,color:C.white,valign:'top'});
}

// 10
{
 const s=base('09 · Lean lane','The quorum proof chain, end to end','The kernel proves finite counting under equal voting power — then exposes the honest-node obligation.','FORMALLY PROVED');
 const eqs=[
  ['COMMITTEE','n = 3f + 1',C.violet],['QUORUM','q = 2f + 1',C.cyan],['INTERSECTION','|Q₁ ∩ Q₂| ≥ 2q − n',C.amber],['SUBSTITUTE','≥ f + 1',C.green]
 ];
 eqs.forEach((a,i)=>{const x=.7+i*3.08; rect(s,x,1.72,2.65,1.2,C.bg2,.12,a[2]); addText(s,a[0],x+.16,1.92,2.33,.2,{fontSize:10,bold:true,color:a[2],align:'center'}); addText(s,a[1],x+.16,2.25,2.33,.34,{fontSize:19,bold:true,color:C.white,align:'center'}); if(i<3)s.addShape(pptx.ShapeType.chevron,{x:x+2.72,y:2.15,w:.28,h:.34,fill:{color:C.line},line:{color:C.line}});});
 rect(s,1.15,3.55,11.05,1.12,C.card2,.12,C.green);
 addText(s,'At most f Byzantine',1.45,3.84,2.4,.28,{fontSize:16,bold:true,color:C.amber}); addText(s,'intersection has at least f + 1',4.05,3.82,3.2,.3,{fontSize:17,bold:true,color:C.white}); addText(s,'⇒ ≥ 1 honest validator',7.48,3.76,3.95,.4,{fontSize:22,bold:true,color:C.green,align:'center'});
 rect(s,2.0,5.18,9.25,.78,C.bg2,.1,C.line); addText(s,'GLOBAL COUNTING',2.3,5.4,1.55,.2,{fontSize:10,bold:true,color:C.cyan}); addText(s,'reduces safety to a local invariant:',3.92,5.34,2.8,.28,{fontSize:14,color:C.text}); addText(s,'an honest validator does not double-support.',6.7,5.32,4.1,.3,{fontSize:15,bold:true,color:C.white});
}

// 11
{
 const s=base('10 · theorem coverage','Exact Lean coverage — theorem by theorem','Names are part of the audit trail. Green states what is proved; the right edge states what is not.','FORMALLY PROVED');
 const th=[
  ['quorum_intersection_at_least_f_add_one','Two 2f+1 quorums inside 3f+1 overlap by ≥ f+1.'],
  ['quorum_intersection_contains_honest','With ≤ f Byzantine, the intersection contains an honest validator.'],
  ['lemma5_at_most_one_certified_block_per_slot','Same-slot certificates coincide, given honest single-support.'],
  ['conflicting_transactions_cannot_both_gather_quorums','Conflicting transactions cannot both gather quorums, given honest no-double-vote.']
 ];
 th.forEach((a,i)=>{const y=1.63+i*1.03; rect(s,.65,y,8.55,.8,C.bg2,.1,C.green); addText(s,a[0],.87,y+.1,4.55,.22,{fontFace:'Consolas',fontSize:11.5,bold:true,color:C.green}); addText(s,a[1],5.45,y+.08,3.42,.44,{fontSize:12.5,color:C.text,valign:'top'});});
 rect(s,9.52,1.63,3.12,3.89,'241B2A',.12,C.red); addText(s,'DOES NOT PROVE',9.78,1.88,2.55,.25,{fontSize:11,bold:true,color:C.red,align:'center'});
 addText(s,'DAG traversal\nDirect + indirect decisions\nNetworking + signatures\nLiveness + epoch change\nRust equivalence',9.92,2.38,2.22,2.2,{fontSize:12.5,color:C.text,valign:'top',align:'center'});
 rect(s,.65,5.92,11.99,.5,C.black,.08,C.line); addText(s,'PROOF HYGIENE',.88,6.05,1.65,.18,{fontSize:9.5,bold:true,color:C.cyan}); addText(s,'PASSED — no sorry · admit · axiom · unsafe · native_decide',2.55,6.0,7.25,.25,{fontFace:'Consolas',fontSize:13,bold:true,color:C.white}); tag(s,'LEAN BUILD: 3001 JOBS',10.15,6.03,C.green,2.1);
}

// 12
{
 const s=base('11 · threshold map','Safety envelope: where honest overlap survives','This chart is exact for the equal-authority abstraction; the margin is max(0, 2q − n − b).','FORMALLY PROVED');
 rect(s,.62,1.62,8.2,4.95,'FFFFFF',.12,'D5E1EB'); s.addImage({path:AS.safety,...contain(AS.safety,.82,1.8,7.8,4.56)});
 cardText(s,9.15,1.73,3.45,1.1,'READ IT','Positive margin ⇒ at least one honest validator remains in quorum overlap.',C.green);
 cardText(s,9.15,3.05,3.45,1.05,'BOUNDARY','Zero margin ⇒ this counting argument no longer guarantees honest overlap.',C.amber);
 cardText(s,9.15,4.32,3.45,1.3,'LIMITATION','Production Sui uses stake-weighted voting power. This figure counts equal authorities.',C.red);
 tag(s,'EQUAL AUTHORITY ONLY',9.75,6.05,C.red,2.25);
}

// 13
{
 const s=base('12 · sensitivity, not forecast','Fault sensitivity: useful shape, deliberately uncalibrated','The model asks “how does the response move?” — never “what will production latency be?”','SYNTHETIC SENSITIVITY');
 rect(s,.62,1.62,8.25,4.95,'FFFFFF',.12,'D5E1EB'); s.addImage({path:AS.fault,...contain(AS.fault,.83,1.8,7.83,4.56)});
 rect(s,9.12,1.72,3.5,3.32,C.bg2,.12,C.violet); addText(s,'MODEL INPUTS',9.4,1.98,2.85,.24,{fontSize:11,bold:true,color:C.violet});
 addText(s,'3 × base delay\n+ Gaussian jitter\n+ 45 ms / crash\n+ 25 ms / Byzantine\n≥ 250 seeded samples',9.42,2.4,2.8,1.75,{fontFace:'Consolas',fontSize:15,color:C.white,valign:'top'});
 rect(s,9.12,5.28,3.5,.95,'241B2A',.12,C.red); addText(s,'NOT A WAN SIMULATION\nNOT A PRODUCTION PREDICTION',9.38,5.52,2.95,.42,{fontSize:12,bold:true,color:C.red,align:'center'});
}

// 14
{
 const s=base('13 · reported benchmark','Production comparison: faithful transcription, not reproduction','NDSS 2025 Table I · committee 137 · offered load 5,000 TPS. Values are checked against a local CSV fixture.','CSV TRANSCRIPTION');
 rect(s,.62,1.62,8.15,4.95,'FFFFFF',.12,'D5E1EB'); s.addImage({path:AS.prod,...contain(AS.prod,.82,1.8,7.75,4.56)});
 rect(s,9.05,1.72,3.55,1.35,C.bg2,.12,C.amber); addText(s,'BULLSHARK',9.33,1.94,2.95,.22,{fontSize:11,bold:true,color:C.amber}); addText(s,'P50  2890 ms\nP95  4600 ms',9.34,2.28,2.86,.52,{fontSize:17,bold:true,color:C.white,valign:'top'});
 rect(s,9.05,3.28,3.55,1.35,C.bg2,.12,C.green); addText(s,'MYSTICETI-C',9.33,3.5,2.95,.22,{fontSize:11,bold:true,color:C.green}); addText(s,'P50   650 ms\nP95   975 ms',9.34,3.84,2.86,.52,{fontSize:17,bold:true,color:C.white,valign:'top'});
 rect(s,9.05,4.84,3.55,1.36,'241B2A',.12,C.red); addText(s,'CLAIM LIMIT',9.33,5.08,2.95,.2,{fontSize:10,bold:true,color:C.red}); addText(s,'CSV consistency only. No independent deployment or measurement.',9.33,5.38,2.86,.5,{fontSize:13,bold:true,color:C.white,valign:'top'});
}

// 15
{
 const s=base('14 · engineering translation','What developers should take into code review','The theorem is small. The implementation obligations it exposes are concrete.','PRACTICAL IMPLICATIONS');
 const checks=[
  ['01','COUNT AUTHORITY','Never count blocks or messages as independent voting power.',C.cyan],
  ['02','SEPARATE AXES','Crash-performance sensitivity is not Byzantine safety.',C.violet],
  ['03','LOCAL OBLIGATION','Persist “no double-support / no conflicting vote” across recovery.',C.green],
  ['04','WEIGHTED REALITY','Production Sui uses delegated-stake voting power > 2/3.',C.amber],
  ['05','TRACE ADAPTER PATH','Builder/relay-style timestamp + order traces → provenance → revision-pinned differential tests. No Ethereum builder flow or Sui production implementation claim.',C.red]
 ];
 checks.forEach((a,i)=>{const x=.68+(i%2)*6.1,y=1.63+Math.floor(i/2)*1.35,w=i===4?12.0:5.7; rect(s,x,y,w,1.02,C.bg2,.1,a[3]); addText(s,a[0],x+.18,y+.18,.52,.35,{fontSize:19,bold:true,color:a[3],align:'center'}); line(s,x+.85,y+.17,0,.65,C.line,1); addText(s,a[1],x+1.05,y+.14,1.72,.22,{fontSize:10,bold:true,color:a[3]}); addText(s,a[2],x+2.8,y+.10,w-3.02,.48,{fontSize:13.5,bold:true,color:C.white,valign:'top'});});
}

// 16
{
 const s=base('15 · reproducibility','One command. Three lanes. One bounded release gate.','Rust executes and replays; Wolfram conforms and visualizes; Lean checks the mapped theorem.','REPRODUCIBLE BUILD');
 rect(s,.7,1.62,11.95,.75,C.black,.1,C.cyan); addText(s,'PS> Set-Location "C:\\Amp_demos\\Mysticeti-Consensus-Digital-Twin"; .\\build_all.ps1',.98,1.84,11.3,.28,{fontFace:'Consolas',fontSize:14,bold:true,color:C.cyan});
 const stages=[['RUST','26 tests · clippy × 2 OS',C.amber],['CAMPAIGN','jobs1 = jobs8 hash',C.cyan],['LEAN 4','3001 jobs · scan pass',C.green],['EVIDENCE GATE','audit 29/0 · PASSED',C.violet]];
 stages.forEach((a,i)=>{const x=.72+i*3.05; rect(s,x,2.78,2.72,1.38,C.bg2,.12,a[2]); addText(s,a[0],x+.16,3.02,2.4,.24,{fontSize:12,bold:true,color:a[2],align:'center'}); addText(s,a[1],x+.16,3.42,2.4,.32,{fontSize:13.5,color:C.text,align:'center'}); if(i<3)s.addShape(pptx.ShapeType.chevron,{x:x+2.78,y:3.22,w:.22,h:.5,fill:{color:C.line},line:{color:C.line}});});
 const stats=[['26','Rust tests',C.amber],['5fdc…a6a','jobs1 = jobs8',C.cyan],['12 / 12','Wolfram',C.violet],['29 / 0','audit pass/fail',C.green],['PASS','Lean proof hygiene',C.green]];
 stats.forEach((a,i)=>{const x=.7+i*2.48; rect(s,x,4.82,2.18,1.05,C.card2,.1,a[2]); addText(s,a[0],x+.12,4.98,1.94,.32,{fontSize:20,bold:true,color:a[2],align:'center'}); addText(s,a[1],x+.12,5.42,1.94,.22,{fontSize:10,bold:true,color:C.text,align:'center'});});
 tag(s,'COMBINED RELEASE GATE: PASSED',4.92,6.25,C.green,3.5);
}

// 17
{
 const s=pptx.addSlide('DARK');
 s.addShape(pptx.ShapeType.rect,{x:0,y:0,w:5.15,h:7.5,fill:{color:C.black},line:{color:C.black}});
 s.addImage({path:AS.daniel,...contain(AS.daniel,0,0,5.15,7.5)});
 s.addShape(pptx.ShapeType.rect,{x:4.25,y:0,w:2.0,h:7.5,fill:{color:C.bg,transparency:15},line:{color:C.bg,transparency:100}});
 tag(s,'LET’S BUILD THE NEXT BRIDGE',5.75,.62,C.green,2.55);
 addText(s,'Bring proofs to\nproduction conversations.',5.75,1.22,6.7,1.45,{fontSize:37,bold:true,color:C.white,valign:'top'});
 addText(s,'Rust executes and replays. Wolfram explores and audits. Lean proves bounded mathematics. Amp orchestrates the evidence gate.',5.78,3.0,6.15,.85,{fontSize:18,color:C.text,valign:'top'});
 addText(s,'Daniel Liezrowice',5.78,4.22,4.0,.35,{fontSize:22,bold:true,color:C.cyan});
 addText(s,'ESL · Engineering Software Lab',5.78,4.62,4.4,.25,{fontSize:13.5,color:C.muted});
 rect(s,5.75,5.18,3.12,.82,'FFFFFF',.1,'D6E2EF'); s.addImage({path:AS.logo,...contain(AS.logo,5.98,5.39,2.65,.38),hyperlink:{url:ESL}});
 addText(s,[{text:ESL,options:{hyperlink:{url:ESL},color:C.cyan,underline:true}}],9.15,5.28,3.0,.28,{fontSize:15,bold:true,color:C.cyan});
 addText(s,[{text:LINKEDIN,options:{hyperlink:{url:LINKEDIN},color:C.cyan,underline:true}}],5.78,6.05,5.2,.28,{fontSize:13.5,bold:true,color:C.cyan});
 addText(s,[{text:REPO,options:{hyperlink:{url:REPO},color:C.green,underline:true}}],5.78,6.40,6.15,.28,{fontSize:12.5,bold:true,color:C.green});
 addText(s,'Independent bounded research demo; not affiliated with or endorsed by Mysten Labs or Sui. Lean proves mapped mathematics, not the Rust binary.',5.78,6.75,6.4,.38,{fontSize:10.5,color:C.text});
 line(s,5.78,7.22,6.38,0,C.line,.7);
}

pptx.writeFile({ fileName: OUT });
