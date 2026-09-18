import {Composition, Interactive, interpolate, useCurrentFrame} from "remotion";
import {Icon,Scene,Touch,LoopReset} from "./Scene";

export function SendButton(){
  const frame=useCurrentFrame();
  const text="See you at golden hour.";
  const count=Math.floor(interpolate(frame,[10,47],[0,text.length],{extrapolateLeft:"clamp",extrapolateRight:"clamp"}));
  const sending=frame>=68&&frame<116;
  const sent=frame>=116&&frame<192;
  return <Scene title="A small hello." number="05" hint="An action. A little wait. A clear result." background="#eef0f1">
    <Interactive.Div name="Message composer" style={{position:"absolute",left:205,top:337,width:670,height:470,padding:42,borderRadius:36,backgroundColor:"#fff",border:"1px solid #e1e4e6",boxShadow:"0 22px 60px #273e4810"}}>
      <div style={{fontSize:22,color:"#969da0",display:"flex",justifyContent:"space-between"}}><span>TO JAMIE</span><span>NEW MESSAGE</span></div>
      <div style={{marginTop:26,padding:28,height:180,borderRadius:24,backgroundColor:"#f5f6f7",fontSize:34,lineHeight:1.45,letterSpacing:-.8,color:count===0?"#bcc1c4":"#343c41"}}>{count===0?"Write a message…":text.slice(0,count)}{frame<63&&<span style={{color:"#8baabd",opacity:Math.floor(frame/12)%2===0?1:0}}>|</span>}</div>
      <Interactive.Div name="Send state button" style={{position:"absolute",right:42,bottom:40,height:78,width:interpolate(frame,[115,130,191,207],[260,192,192,260],{extrapolateLeft:"clamp",extrapolateRight:"clamp"}),borderRadius:50,backgroundColor:sent?"#466f5c":"#222a2e",color:"#fff",display:"flex",alignItems:"center",justifyContent:"center",gap:14,fontSize:28,fontWeight:500,opacity:count===0?.3:1}}>
        {sending?<div style={{width:25,height:25,borderRadius:50,border:"3px solid #ffffff45",borderTopColor:"#fff",rotate:`${(frame-68)*14}deg`}}/>:<Icon name={sent?"check":"arrow"} size={28}/>}{sent?"Sent":sending?"Sending":"Send message"}
      </Interactive.Div>
      {sent&&<div style={{position:"absolute",left:46,bottom:67,fontSize:22,color:"#83968a"}}>Delivered. Nice and easy.</div>}
    </Interactive.Div>
    <Touch x={703} y={729} from={59} to={75}/>
  </Scene>;
}
const SendLoop=()=> <LoopReset><SendButton/></LoopReset>;
export const SendComposition=()=> <Composition id="SendButton" component={SendLoop} width={1080} height={1080} fps={30} durationInFrames={240}/>;
