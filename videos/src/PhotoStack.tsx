import {Composition, Easing, Interactive, interpolate, useCurrentFrame} from "remotion";
import {Artwork, Scene, Touch} from "./Scene";

export function PhotoStack() {
  const frame = useCurrentFrame();
  const spread = interpolate(frame, [34, 66, 144, 184], [0, 1, 1, 0], {
    extrapolateLeft: "clamp", extrapolateRight: "clamp", easing: Easing.bezier(.22, .8, .25, 1),
  });
  const width = 360 - 84 * spread;
  return <Scene title="Photo Stack" number="01" hint="Tap to unfold. Tap to gather." background="#f3f0e9">
    {[2, 1, 0].map(index => <Interactive.Div key={index} name={`Postcard ${index + 1}`} style={{
      position: "absolute", left: 540 - width / 2 + (index - 1) * 244 * spread + index * 12 * (1 - spread),
      top: 330 + (index === 1 ? -18 : 8) * spread - index * 12 * (1 - spread),
      width, height: 440, borderRadius: 26, padding: 7, backgroundColor: "#fff",
      boxShadow: "0 18px 38px #42382c18", rotate: `${[0, -9, 8][index] * (1 - spread)}deg`,
      zIndex: spread > .6 ? index + 2 : 5 - index,
    }}>
      <div style={{position:"relative",width:"100%",height:"100%",borderRadius:20,overflow:"hidden"}}>
        <Artwork kind={index}/>
        <div style={{position:"absolute",left:24,bottom:30,color:"#fff"}}>
          <div style={{fontFamily:"Georgia, serif",fontSize:44,fontWeight:600}}>{["Alpine","Coast","Dune"][index]}</div>
          <div style={{fontSize:13,letterSpacing:1.6,marginTop:10}}>{["SLOW WEEKENDS","TAKE THE LONG WAY","NOWHERE TO RUSH"][index]}</div>
        </div>
      </div>
    </Interactive.Div>)}
    <Touch x={556} y={680} from={25} to={44}/>
    <Touch x={556} y={680} from={135} to={155}/>
  </Scene>;
}
export const PhotoComposition = () => <Composition id="PhotoStack" component={PhotoStack} width={1080} height={1080} fps={30} durationInFrames={240}/>;
