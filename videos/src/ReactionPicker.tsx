import {Composition, Easing, Interactive, interpolate, useCurrentFrame} from "remotion";
import {Icon, Scene, Touch} from "./Scene";

export function ReactionPicker() {
  const frame = useCurrentFrame();
  const open = interpolate(frame,[42,59,132,146],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.22,.8,.25,1)});
  const hover = interpolate(frame,[65,79,93,107],[0,1,1,2],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.22,.8,.25,1)});
  const done = frame >= 136 && frame < 197;
  return <Scene title="Reaction Picker" number="04" hint="Hold, slide, release." background="#f3f0ed">
    <Interactive.Div name="Conversation card" style={{position:"absolute",left:225,top:285,width:630,height:490,padding:44,borderRadius:34,backgroundColor:"#fff",boxShadow:"0 22px 60px #453d3510",border:"1px solid #e8e1db"}}>
      <div style={{display:"flex",gap:18,alignItems:"center"}}><div style={{width:62,height:62,borderRadius:50,backgroundColor:"#f0dcc8",display:"grid",placeItems:"center",fontSize:26,color:"#9d6945"}}>A</div><div><div style={{fontSize:27,fontWeight:600}}>Alex</div><div style={{fontSize:20,color:"#89847d",marginTop:4}}>Just now</div></div></div>
      <div style={{fontSize:36,lineHeight:1.4,letterSpacing:-.7,marginTop:29}}>Made time for<br/>the things I love.</div>
      <div style={{position:"absolute",left:44,bottom:77,height:59,padding:"0 24px",borderRadius:35,backgroundColor:done?"#f7eadb":"#f6f3ef",display:"flex",alignItems:"center",gap:13,fontSize:26,color:"#787269"}}>
        {done ? <><span>😂</span><span style={{fontSize:23}}>1</span></> : <><Icon name="smile"/>React</>}
      </div>
      <div style={{position:"absolute",left:44,bottom:35,fontSize:20,color:"#89847d"}}>{done?"Tap to remove":"Hold to react"}</div>
      <Interactive.Div name="Reaction tray" style={{position:"absolute",left:25,bottom:164,padding:12,display:"flex",gap:2,borderRadius:60,backgroundColor:"#fffcf8",border:"1px solid #ece4dc",boxShadow:"0 15px 40px #55402b18",opacity:open,scale:.92+.08*open,transformOrigin:"left bottom",translate:`0 ${(1-open)*12}px`}}>
        {["❤️","🎉","😂","😮","👍"].map((emoji,index)=>{
          const emphasis = Math.max(0,1-Math.abs(hover-index));
          return <div key={emoji} style={{fontFamily:"Apple Color Emoji, sans-serif",width:82,height:65,display:"grid",placeItems:"center",fontSize:44,scale:1+.28*emphasis,translate:`0 ${-14*emphasis}px`}}>{emoji}</div>;
        })}
      </Interactive.Div>
    </Interactive.Div>
    <Touch x={330+84*hover} y={668} from={30} to={141}/>
    <Touch x={323} y={668} from={188} to={207}/>
  </Scene>;
}
export const ReactionComposition = () => <Composition id="ReactionPicker" component={ReactionPicker} width={1080} height={1080} fps={30} durationInFrames={240}/>;
