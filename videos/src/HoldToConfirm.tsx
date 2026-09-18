import { Composition, Interactive, interpolate, useCurrentFrame } from "remotion";
import {Icon, Scene, Touch} from "./Scene";

export function HoldToConfirm() {
  const frame=useCurrentFrame();
  const complete=frame>=132 && frame<198;
  const progress=interpolate(frame,[0,30,47,59,96,132,198,215,239],[0,0,.43,0,0,1,1,0,0],{extrapolateRight:"clamp"});
  return <Scene title="Hold to Confirm" number="02" hint="Release to cancel. Hold to confirm.">
    <Interactive.Div name="Archive card" style={{position:"absolute",left:240,top:290,width:600,height:495,padding:50,borderRadius:40,backgroundColor:"#fff",border:"1px solid #e5e8e0",boxShadow:"0 28px 65px #3540300b",display:"flex",alignItems:"center",flexDirection:"column"}}>
      <div style={{width:80,height:80,backgroundColor:"#f1f4ee",borderRadius:25,display:"grid",placeItems:"center",color:"#577262",marginBottom:28}}><Icon name="archive" size={38}/></div>
      <div style={{fontSize:34,fontWeight:600,letterSpacing:-1}}>Keep a quieter inbox.</div>
      <div style={{fontSize:23,color:"#858c7f",marginTop:13}}>Archive when you’re ready.</div>
      <Interactive.Div name="Hold progress button" style={{position:"relative",marginTop:42,width:414,height:88,borderRadius:50,backgroundColor:"#eef3ed",overflow:"hidden",display:"flex",justifyContent:"center",alignItems:"center",gap:15,color:"#3d6e59",fontSize:28,fontWeight:600,scale:interpolate(frame,[30,35,47,59,96,100,132,144],[1,.97,.97,1,1,.97,.97,1],{extrapolateLeft:"clamp",extrapolateRight:"clamp"})}}>
        <div style={{position:"absolute",inset:0,width:`${progress*100}%`,backgroundColor:"#b8d3b6"}}/>
        <div style={{position:"relative",display:"flex",alignItems:"center",gap:14}}><Icon name={complete?"check":"archive"} size={29}/>{complete?"Archived":"Hold to archive"}</div>
      </Interactive.Div>
      <div style={{marginTop:20,fontSize:22,color:"#858c7f"}}>{complete?"Collection archived":"Hold for 1.2 seconds"}</div>
    </Interactive.Div>
    <Touch x={647} y={633} from={28} to={51}/><Touch x={647} y={633} from={92} to={140}/>
  </Scene>;
}
export const HoldComposition=()=> <Composition id="HoldToConfirm" component={HoldToConfirm} width={1080} height={1080} fps={30} durationInFrames={240}/>;
