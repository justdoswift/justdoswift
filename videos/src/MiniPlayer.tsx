import {Composition, Easing, Interactive, interpolate, useCurrentFrame} from "remotion";
import {AlbumArtwork, Scene, Touch} from "./Scene";

export function MiniPlayer() {
  const frame = useCurrentFrame();
  const open = interpolate(frame, [35, 68, 171, 209], [0, 1, 1, 0], {
    extrapolateLeft:"clamp", extrapolateRight:"clamp", easing:Easing.bezier(.22,.8,.25,1),
  });
  const playing = frame >= 97 && frame < 148;
  return <Scene title="Mini Player" number="03" hint="One cover. Two views." background="#f0eee9">
    <Interactive.Div name="Expanding player" style={{position:"absolute",left:240,top:665-420*open,width:600,height:114+506*open,borderRadius:30+14*open,backgroundColor:"#fff",boxShadow:"0 24px 50px #55453912",border:"1px solid #e4e0d8",overflow:"hidden"}}>
      <div style={{position:"absolute",left:280,top:17,width:40,height:5,borderRadius:5,backgroundColor:"#dbd8d1",opacity:open}}/>
      <div style={{position:"absolute",left:18+110*open,top:18+31*open,width:78+266*open,height:78+266*open,borderRadius:13+10*open,overflow:"hidden"}}><AlbumArtwork/></div>
      {/* Only the cover morphs; separate text layouts avoid collisions in transit. */}
      <div style={{position:"absolute",left:118+376*open,top:29,opacity:interpolate(open,[0,.045],[1,0],{extrapolateRight:"clamp"}),whiteSpace:"nowrap"}}>
        <div style={{fontSize:28,fontWeight:600,letterSpacing:-.8}}>Soft Focus</div>
        <div style={{fontSize:20,color:"#8c8982",marginTop:8}}>Sunday Sessions</div>
      </div>
      <div style={{position:"absolute",left:0,right:0,top:411,opacity:interpolate(open,[.95,1],[0,1],{extrapolateLeft:"clamp"}),textAlign:"center"}}>
        <div style={{fontSize:38,fontWeight:600,letterSpacing:-.8}}>Soft Focus</div>
        <div style={{fontSize:22,color:"#8c8982",marginTop:8}}>Sunday Sessions</div>
      </div>
      <div style={{position:"absolute",left:518-238*open,top:37+513*open,width:40,height:40,display:"grid",placeItems:"center",scale:1+.2*open}}>
        {playing ? <div style={{display:"flex",gap:8}}><span style={{width:8,height:26,borderRadius:2,backgroundColor:"#262a26"}}/><span style={{width:8,height:26,borderRadius:2,backgroundColor:"#262a26"}}/></div> : <svg width="32" height="32" viewBox="0 0 32 32"><path d="M9 5 28 16 9 27Z" fill="#262a26"/></svg>}
      </div>
      <div style={{position:"absolute",left:75,top:514,width:450,height:3,backgroundColor:"#eee9e1",opacity:interpolate(open,[.95,1],[0,1],{extrapolateLeft:"clamp"})}}>
        <div style={{height:3,width:interpolate(frame,[97,148],[55,126],{extrapolateLeft:"clamp",extrapolateRight:"clamp"}),backgroundColor:"#ab7760"}}/>
      </div>
    </Interactive.Div>
    <Touch x={410} y={722} from={26} to={44}/>
    <Touch x={540} y={815} from={89} to={108}/>
    <Touch x={540} y={815} from={140} to={157}/>
    <Touch x={540} y={265} from={163} to={182}/>
  </Scene>;
}
export const PlayerComposition = () => <Composition id="MiniPlayer" component={MiniPlayer} width={1080} height={1080} fps={30} durationInFrames={240}/>;
