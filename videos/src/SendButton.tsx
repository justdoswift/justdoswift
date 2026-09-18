import {Composition, Easing, Interactive, interpolate, useCurrentFrame} from "remotion";
import {Icon, Scene, Touch} from "./Scene";

export function SendButton() {
  const frame = useCurrentFrame();
  const sending = frame >= 48 && frame < 94;
  const success = interpolate(frame,[94,104,174,190],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.22,.8,.25,1)});
  return <Scene title="Send Button" number="05" hint="Send. Wait. Confirm." background="#eef0f1">
    <Interactive.Div name="Message composer" style={{position:"absolute",left:205,top:300,width:670,height:470,padding:42,borderRadius:36,backgroundColor:"#fff",border:"1px solid #e1e4e6",boxShadow:"0 22px 60px #273e4810"}}>
      <div style={{fontSize:22,color:"#858d91",display:"flex",justifyContent:"space-between"}}><span>TO JAMIE</span><span>DRAFT</span></div>
      <div style={{marginTop:26,padding:28,height:180,borderRadius:24,backgroundColor:"#f5f6f7",fontSize:34,lineHeight:1.45,letterSpacing:-.8,color:"#343c41"}}>See you at golden hour.</div>
      <Interactive.Div name="Send state button" style={{position:"absolute",right:42,bottom:40,height:78,width:260-68*success,borderRadius:50,backgroundColor:"#222a2e",color:"#fff",fontSize:28,fontWeight:500,overflow:"hidden"}}>
        <div style={{position:"absolute",inset:0,backgroundColor:"#466f5c",opacity:success}}/>
        <div style={{position:"absolute",inset:0,display:"flex",alignItems:"center",justifyContent:"center",gap:14,opacity:1-success,whiteSpace:"nowrap"}}>
          {sending?<div style={{width:25,height:25,borderRadius:50,border:"3px solid #ffffff45",borderTopColor:"#fff",rotate:`${(frame-48)*14}deg`}}/>:<Icon name="arrow" size={28}/>}
          {sending?"Sending":"Send message"}
        </div>
        <div style={{position:"absolute",inset:0,display:"flex",alignItems:"center",justifyContent:"center",gap:14,opacity:success,translate:`0 ${8*(1-success)}px`}}><Icon name="check" size={28}/>Sent</div>
      </Interactive.Div>
      <div style={{position:"absolute",left:46,bottom:67,fontSize:22,color:"#6f8979",opacity:success}}>Message sent</div>
    </Interactive.Div>
    <Touch x={703} y={691} from={38} to={58}/>
  </Scene>;
}
export const SendComposition = () => <Composition id="SendButton" component={SendButton} width={1080} height={1080} fps={30} durationInFrames={240}/>;
