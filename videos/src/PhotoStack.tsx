import {Composition, Easing, Interactive, interpolate, useCurrentFrame} from "remotion";
import {Artwork, Scene, Touch, LoopReset} from "./Scene";

export function PhotoStack(){
  const frame=useCurrentFrame();
  const spread=interpolate(frame,[35,65,125,155],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp",easing:Easing.bezier(.2,.9,.25,1)});
  const swapped=frame>=194;
  return <Scene title="Somewhere, slowly." number="02" hint="Tap to unfold. Swipe to rediscover." background="#f3f0e9">
    <div style={{position:"absolute",top:295,left:80,right:80,textAlign:"center",fontSize:25,color:"#8c8d81"}}>THREE PLACES WORTH KEEPING</div>
    {[2,1,0].map(index=>{
      const relative=swapped?(index+2)%3:index;
      return <Interactive.Div key={index} name={`Postcard ${index+1}`} style={{position:"absolute",left:360+(index-1)*230*spread+relative*15*(1-spread)+(index===0?interpolate(frame,[171,190,194,216],[0,145,145,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp"}):0),top:400+(index===1?-18:8)*spread-relative*12*(1-spread),width:360-70*spread,height:420,borderRadius:26,padding:7,backgroundColor:"#fff",boxShadow:"0 22px 45px #42382c20",rotate:`${([0,-9,8][relative])*(1-spread)}deg`,zIndex:spread>.5?index+2:5-relative}}>
        <div style={{position:"relative",width:"100%",height:"100%",borderRadius:20,overflow:"hidden"}}><Artwork kind={index}/><div style={{position:"absolute",left:28,bottom:35,color:"#fff"}}><div style={{fontFamily:"Georgia, serif",fontSize:47,fontWeight:600}}>{["Alpine","Coast","Dune"][index]}</div><div style={{fontSize:14,letterSpacing:2,marginTop:10}}>{["SLOW WEEKENDS","TAKE THE LONG WAY","NOWHERE TO RUSH"][index]}</div></div></div>
      </Interactive.Div>;
    })}
    <Touch x={550} y={665} from={26} to={43}/><Touch x={550} y={665} from={118} to={135}/>
    <Touch x={interpolate(frame,[170,196],[540,690],{extrapolateLeft:"clamp",extrapolateRight:"clamp"})} y={680} from={170} to={199}/>
  </Scene>;
}
const PhotoLoop=()=> <LoopReset><PhotoStack/></LoopReset>;
export const PhotoComposition=()=> <Composition id="PhotoStack" component={PhotoLoop} width={1080} height={1080} fps={30} durationInFrames={240}/>;
