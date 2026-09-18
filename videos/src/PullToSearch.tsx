import {Composition, Easing, Interactive, interpolate, useCurrentFrame} from "remotion";
import {Icon, Scene, Touch} from "./Scene";

export function PullToSearch() {
  const frame = useCurrentFrame();
  const reveal = interpolate(frame,[47,73,180,211],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.22,.8,.25,1)});
  const pulled = interpolate(frame,[23,47,73],[0,80,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp"});
  const filtered = interpolate(frame,[94,111,178,198],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.22,.8,.25,1)});
  const query = frame < 180 ? "weekend".slice(0,Math.floor(interpolate(frame,[90,117],[0,7],{extrapolateLeft:"clamp",extrapolateRight:"clamp"}))) : "";
  const rows = ["Morning pages","Weekend places","Small ideas","Things to make"];
  return <Scene title="Pull to Search" number="06" hint="Pull to reveal. Type to find." background="#f1f1ed">
    <Interactive.Div name="Saved collection" style={{position:"absolute",left:220,top:235,width:640,height:610,borderRadius:36,backgroundColor:"#fff",border:"1px solid #e2e3dd",boxShadow:"0 22px 60px #343c2d10",overflow:"hidden"}}>
      <div style={{padding:"32px 30px 25px",fontSize:36,fontWeight:600,letterSpacing:-1,display:"flex",justifyContent:"space-between",alignItems:"center"}}><span>Saved for later</span><span style={{opacity:1-reveal}}><Icon name="search" size={27}/></span></div>
      <div style={{position:"absolute",top:117,left:30,right:30,height:60,display:"flex",alignItems:"center",gap:18,opacity:reveal,translate:`0 ${-12*(1-reveal)}px`}}>
        <div style={{height:60,flex:1,borderRadius:15,backgroundColor:"#f1f2ee",display:"flex",alignItems:"center",gap:13,padding:"0 16px",fontSize:25,color:query?"#47523f":"#858d7e"}}>
          <Icon name="search" size={24}/><span>{query || "Search collections"}</span>
        </div>
        <span style={{fontSize:23,color:"#67745d"}}>Cancel</span>
      </div>
      <div style={{position:"absolute",top:113,left:306,opacity:(pulled/80)*(1-reveal),color:"#8a9482"}}><Icon name="search" size={25}/></div>
      <div style={{position:"absolute",top:117+82*reveal+pulled,left:30,right:30}}>
        {rows.map((item,index)=><div key={item} style={{position:"absolute",left:0,right:0,top:index*108*(1-filtered),height:95,display:"flex",alignItems:"center",gap:19,padding:"13px 16px",border:"1px solid #eceee7",borderRadius:19,backgroundColor:"#fff",opacity:index===1?1:1-filtered,zIndex:index===1?2:1}}>
          <div style={{height:51,width:51,borderRadius:14,backgroundColor:["#f2e4cf","#dfe9df","#e4e1ed","#e7e8df"][index],display:"grid",placeItems:"center",color:"#8b8977"}}><Icon name="bookmark" size={24}/></div>
          <div><div style={{fontSize:26,fontWeight:500}}>{item}</div><div style={{fontSize:19,color:"#858e7b",marginTop:6}}>A little collection</div></div>
          <div style={{marginLeft:"auto",color:"#a9b09f"}}><Icon name="right" size={18}/></div>
        </div>)}
        <div style={{position:"absolute",top:125,left:0,right:0,fontSize:22,color:"#858e7b",textAlign:"center",opacity:filtered}}>1 collection found</div>
      </div>
    </Interactive.Div>
    <Touch x={550} y={interpolate(frame,[24,48],[440,550],{extrapolateLeft:"clamp",extrapolateRight:"clamp"})} from={23} to={54}/>
    <Touch x={788} y={382} from={171} to={190}/>
  </Scene>;
}
export const SearchComposition = () => <Composition id="PullToSearch" component={PullToSearch} width={1080} height={1080} fps={30} durationInFrames={240}/>;
