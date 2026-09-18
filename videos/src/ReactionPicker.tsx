import {Composition, Easing, Interactive, interpolate, useCurrentFrame} from "remotion";
import {Icon,Scene,Touch,LoopReset} from "./Scene";

export function ReactionPicker(){
  const frame=useCurrentFrame();
  const open=interpolate(frame,[42,57,130,143],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.16,1,.3,1)});
  const selected=frame<73?0:frame<93?1:2;
  const done=frame>=136&&frame<210;
  return <Scene title="A little more feeling." number="04" hint="Hold, slide, release. Say it your way." background="#f3f0ed">
    <Interactive.Div name="Conversation card" style={{position:"absolute",left:225,top:323,width:630,height:490,padding:44,borderRadius:34,backgroundColor:"#fff",boxShadow:"0 22px 60px #453d3510",border:"1px solid #e8e1db"}}>
      <div style={{display:"flex",gap:18,alignItems:"center"}}><div style={{width:62,height:62,borderRadius:50,backgroundColor:"#f0dcc8",display:"grid",placeItems:"center",fontSize:26,color:"#9d6945"}}>A</div><div><div style={{fontSize:27,fontWeight:600}}>Alex</div><div style={{fontSize:20,color:"#a6a199",marginTop:4}}>Just now</div></div></div>
      <div style={{fontSize:36,lineHeight:1.4,letterSpacing:-.7,marginTop:29,maxWidth:480}}>Made a little time for<br/>the things I love.</div>
      <div style={{position:"absolute",left:44,bottom:77,height:59,padding:"0 24px",borderRadius:35,backgroundColor:"#f6f3ef",display:"flex",alignItems:"center",gap:13,fontSize:26,color:"#787269"}}><Icon name="smile"/>{done?"😂":"React"}</div>
      <div style={{position:"absolute",left:44,bottom:35,fontSize:20,color:"#a6a199"}}>{done?"You reacted with laughter.":"A small gesture goes a long way."}</div>
      <Interactive.Div name="Reaction tray" style={{position:"absolute",left:25,bottom:164,padding:12,display:"flex",gap:2,borderRadius:60,backgroundColor:"#fffcf8",border:"1px solid #ece4dc",boxShadow:"0 15px 40px #55402b20",opacity:open,scale:.85+.15*open,transformOrigin:"left bottom",translate:`0 ${(1-open)*15}px`}}>
        {["❤️","🎉","😂","😮","👍"].map((emoji,index)=><div key={emoji} style={{fontFamily:"Apple Color Emoji, sans-serif",width:82,height:65,display:"grid",placeItems:"center",fontSize:44,scale:selected===index?1.32:1,translate:selected===index?"0 -15px":"0 0"}}>{emoji}</div>)}
      </Interactive.Div>
    </Interactive.Div>
    <Touch x={interpolate(frame,[29,59,73,93,130],[330,330,412,496,496],{extrapolateLeft:"clamp",extrapolateRight:"clamp"})} y={707} from={28} to={139}/>
  </Scene>;
}
const ReactionLoop=()=> <LoopReset><ReactionPicker/></LoopReset>;
export const ReactionComposition=()=> <Composition id="ReactionPicker" component={ReactionLoop} width={1080} height={1080} fps={30} durationInFrames={240}/>;
