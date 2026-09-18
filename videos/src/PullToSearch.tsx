import {Composition,Easing,Interactive,interpolate,useCurrentFrame} from "remotion";
import {Icon,Scene,Touch,LoopReset} from "./Scene";

export function PullToSearch(){
  const frame=useCurrentFrame();
  const reveal=interpolate(frame,[47,69,180,205],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.2,.9,.2,1)});
  const pulled=interpolate(frame,[23,47,69],[0,88,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp"});
  const query="weekend".slice(0,Math.floor(interpolate(frame,[90,116],[0,7],{extrapolateLeft:"clamp",extrapolateRight:"clamp"})));
  const filtering=frame>=91&&frame<181;
  const rows=["Morning pages","Weekend places","Small ideas","Things to make"];
  return <Scene title="Find your little things." number="06" hint="Pull down. Find something worth revisiting." background="#f1f1ed">
    <Interactive.Div name="Saved collection" style={{position:"absolute",left:236,top:285,width:608,height:575,borderRadius:36,backgroundColor:"#fff",border:"1px solid #e2e3dd",boxShadow:"0 22px 60px #343c2d10",overflow:"hidden"}}>
      <div style={{padding:"33px 34px 25px",fontSize:33,fontWeight:600,letterSpacing:-1,display:"flex",justifyContent:"space-between",alignItems:"center"}}><span>Saved for later</span><Icon name="search" size={26}/></div>
      <div style={{position:"absolute",top:112,left:30,right:30,height:60,borderRadius:15,backgroundColor:"#f1f2ee",display:"flex",alignItems:"center",gap:14,padding:"0 20px",color:"#7e8778",fontSize:24,opacity:reveal,translate:`0 ${-15*(1-reveal)}px`}}><Icon name="search" size={24}/><span>{filtering?query:"Search your collection"}</span><span style={{marginLeft:"auto",fontSize:19}}>Cancel</span></div>
      {!filtering&&<div style={{position:"absolute",top:115,left:286,opacity:pulled/88,color:"#8a9482"}}><Icon name="search" size={25}/></div>}
      <div style={{position:"absolute",top:117+76*reveal+pulled,left:30,right:30,display:"flex",flexDirection:"column",gap:13}}>
        {rows.filter(item=>!filtering||item.toLowerCase().includes(query)).map((item,index)=><div key={item} style={{height:87,display:"flex",alignItems:"center",gap:19,padding:"13px 16px",border:"1px solid #eceee7",borderRadius:19}}><div style={{height:51,width:51,borderRadius:14,backgroundColor:["#f2e4cf","#dfe9df","#e4e1ed","#e7e8df"][index],display:"grid",placeItems:"center",color:"#8b8977"}}><Icon name="bookmark" size={24}/></div><div><div style={{fontSize:24,fontWeight:500}}>{item}</div><div style={{fontSize:17,color:"#a0a498",marginTop:6}}>A little collection</div></div><div style={{marginLeft:"auto",color:"#b9bcb3"}}><Icon name="right" size={18}/></div></div>)}
        {filtering&&<div style={{fontSize:21,color:"#a0a498",textAlign:"center",paddingTop:22}}>1 collection, just where you left it.</div>}
      </div>
    </Interactive.Div>
    <Touch x={548} y={interpolate(frame,[24,48],[455,562],{extrapolateLeft:"clamp",extrapolateRight:"clamp"})} from={23} to={54}/><Touch x={762} y={429} from={174} to={187}/>
  </Scene>;
}
const SearchLoop=()=> <LoopReset><PullToSearch/></LoopReset>;
export const SearchComposition=()=> <Composition id="PullToSearch" component={SearchLoop} width={1080} height={1080} fps={30} durationInFrames={240}/>;
