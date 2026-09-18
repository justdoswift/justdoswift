import {Composition, Easing, Interactive, interpolate, useCurrentFrame} from "remotion";
import {AlbumArtwork, Scene, Touch, LoopReset} from "./Scene";

export function MiniPlayer(){
  const frame=useCurrentFrame();
  const open=interpolate(frame,[35,64,171,203],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.16,1,.3,1)});
  const playing=frame>=99&&frame<221;
  return <Scene title="Stay in the moment." number="03" hint="One cover. One continuous transition." background="#f0eee9">
    <div style={{position:"absolute",left:250,top:306,fontSize:26,color:"#9b978e"}}>YOUR AFTERNOON, A LITTLE SOFTER.</div>
    <Interactive.Div name="Expanding player" style={{position:"absolute",left:240,top:715-430*open,width:600,height:114+506*open,borderRadius:30+14*open,backgroundColor:"#fff",boxShadow:"0 28px 60px #55453912",border:"1px solid #e4e0d8",overflow:"hidden"}}>
      <div style={{position:"absolute",left:280,top:17,width:40,height:5,borderRadius:5,backgroundColor:"#dbd8d1",opacity:open}}/>
      <div style={{position:"absolute",left:18+110*open,top:18+31*open,width:78+266*open,height:78+266*open,borderRadius:13+10*open,overflow:"hidden"}}><AlbumArtwork/></div>
      <div style={{position:"absolute",left:118+(300-118)*open,top:31+380*open,translate:`${-50*open}% 0`,whiteSpace:"nowrap",textAlign:open>.5?"center":"left"}}><div style={{fontSize:28+10*open,fontWeight:600,letterSpacing:-.8}}>Soft Focus</div><div style={{fontSize:20+2*open,color:"#97948d",marginTop:8}}>Sunday Sessions</div></div>
      <div style={{position:"absolute",left:518-238*open,top:40+510*open,width:40,height:40,display:"grid",placeItems:"center",scale:1+.2*open}}>{playing?<div style={{display:"flex",gap:8}}><span style={{width:8,height:26,borderRadius:2,backgroundColor:"#262a26"}}/><span style={{width:8,height:26,borderRadius:2,backgroundColor:"#262a26"}}/></div>:<svg width="32" height="32" viewBox="0 0 32 32"><path d="M9 5 28 16 9 27Z" fill="#262a26"/></svg>}</div>
      <div style={{position:"absolute",left:75,top:514,width:450,height:3,backgroundColor:"#eee9e1",opacity:open}}><div style={{height:3,width:interpolate(frame,[99,170],[55,126],{extrapolateLeft:"clamp",extrapolateRight:"clamp"}),backgroundColor:"#ab7760"}}/></div>
    </Interactive.Div>
    <Touch x={410} y={774} from={26} to={44}/><Touch x={540} y={855} from={92} to={108}/><Touch x={540} y={310} from={161} to={178}/>
  </Scene>;
}
const PlayerLoop=()=> <LoopReset><MiniPlayer/></LoopReset>;
export const PlayerComposition=()=> <Composition id="MiniPlayer" component={PlayerLoop} width={1080} height={1080} fps={30} durationInFrames={240}/>;
