import { Composition, Interactive, interpolate, useCurrentFrame } from "remotion";
import {Icon, Scene, Touch, LoopReset} from "./Scene";

export function HoldToConfirm() {
  const frame=useCurrentFrame();
  const complete=frame>=142 && frame<202;
  const progress=interpolate(frame,[0,30,47,55,95,142,201,212,239],[0,0,.43,0,0,1,1,0,0],{extrapolateRight:"clamp"});
  return <Scene title="Make it intentional." number="01" hint={frame<70 ? "Release early. Nothing happens." : "Hold to confirm. A little more certain."}>
    <Interactive.Div name="Archive card" style={{position:"absolute",left:240,top:315,width:600,height:495,padding:50,borderRadius:40,backgroundColor:"#fff",border:"1px solid #e5e8e0",boxShadow:"0 28px 65px #3540300b",display:"flex",alignItems:"center",flexDirection:"column"}}>
      <div style={{width:80,height:80,backgroundColor:"#f1f4ee",borderRadius:25,display:"grid",placeItems:"center",color:"#577262",marginBottom:28}}><Icon name="archive" size={38}/></div>
      <div style={{fontSize:34,fontWeight:600,letterSpacing:-1}}>Keep a quieter inbox.</div>
      <div style={{fontSize:23,color:"#959a91",marginTop:13}}>Archive this collection when you’re ready.</div>
      <Interactive.Div name="Hold progress button" style={{position:"relative",marginTop:42,width:414,height:88,borderRadius:50,backgroundColor:"#eef3ed",overflow:"hidden",display:"flex",justifyContent:"center",alignItems:"center",gap:15,color:"#3d6e59",fontSize:28,fontWeight:600,scale:interpolate(frame,[30,35,47,55,95,101,142,152],[1,.97,.97,1,1,.97,.97,1],{extrapolateLeft:"clamp",extrapolateRight:"clamp"})}}>
        <div style={{position:"absolute",inset:0,width:`${progress*100}%`,backgroundColor:"#b8d3b6"}}/>
        <div style={{position:"relative",display:"flex",alignItems:"center",gap:14}}><Icon name={complete?"check":"archive"} size={29}/>{complete?"Archived":"Hold to archive"}</div>
      </Interactive.Div>
      <div style={{marginTop:20,fontSize:20,color:"#939a8e"}}>{complete?"All done. A little less clutter.":"Press and hold for 1.2 seconds"}</div>
    </Interactive.Div>
    <Touch x={647} y={690} from={28} to={51}/><Touch x={647} y={690} from={92} to={150}/>
  </Scene>;
}
const HoldLoop=()=> <LoopReset><HoldToConfirm/></LoopReset>;
export const HoldComposition=()=> <Composition id="HoldToConfirm" component={HoldLoop} width={1080} height={1080} fps={30} durationInFrames={240}/>;
