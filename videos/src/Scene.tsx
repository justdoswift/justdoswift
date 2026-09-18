import React from "react";
import { AbsoluteFill, Interactive, interpolate, useCurrentFrame } from "remotion";

export const Scene: React.FC<{title: string; number: string; hint: string; background?: string; children: React.ReactNode}> = ({title, number, hint, background = "#f3f3ef", children}) => (
  <AbsoluteFill style={{backgroundColor: background, color: "#252723", fontFamily: "Arial, Helvetica, sans-serif"}}>
    <Interactive.Div name="Component label" style={{position:"absolute", top:64, left:64, right:64, display:"flex", justifyContent:"space-between", alignItems:"center", color:"#74776f"}}>
      <span style={{fontSize:30,fontWeight:500,letterSpacing:-.5}}>{title}</span><span style={{fontSize:20,letterSpacing:1}}>STUDY {number}</span>
    </Interactive.Div>
    <AbsoluteFill style={{scale:1.12,transformOrigin:"50% 54%"}}>{children}</AbsoluteFill>
    <Interactive.Div name="Interaction caption" style={{position:"absolute", bottom:103, left:64, right:64, textAlign:"center", fontSize:26, color:"#697064"}}>
      {hint}
    </Interactive.Div>
    <div style={{position:"absolute",bottom:48,left:64,right:64,display:"flex",justifyContent:"space-between",fontSize:18,letterSpacing:1,color:"#92968c"}}><span>JUST DO SWIFT</span><span>REMOTION DEMO</span></div>
  </AbsoluteFill>
);

const paths: Record<string,string> = {
  check:"M5 12l4 4L19 6", arrow:"M12 19V5m-6 6 6-6 6 6", down:"m6 9 6 6 6-6", plus:"M12 5v14M5 12h14",
  search:"M21 21l-5-5M18 10a8 8 0 1 1-16 0 8 8 0 0 1 16 0", archive:"M4 8h16v12H4zM3 4h18v4H3zM9 12h6",
  bookmark:"M6 3h12v18l-6-4-6 4z", right:"m9 5 7 7-7 7", close:"m6 6 12 12M6 18 18 6",
  smile:"M8 14s1.5 3 4 3 4-3 4-3M8 9h.01M16 9h.01M22 12a10 10 0 1 1-20 0 10 10 0 0 1 20 0",
};
export const Icon: React.FC<{name:string; size?:number; color?:string}> = ({name,size=28,color="currentColor"}) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke={color} strokeWidth={1.7} strokeLinecap="round" strokeLinejoin="round"><path d={paths[name] ?? paths.check}/></svg>;

export const Touch: React.FC<{x:number;y:number;from:number;to:number}> = ({x,y,from,to}) => {
  const frame=useCurrentFrame();
  const edge=Math.min(5,(to-from)/3);
  return <div style={{position:"absolute",left:x-24,top:y-24,width:48,height:48,borderRadius:50,border:"2px solid #676b6570",backgroundColor:"#ffffff70",boxShadow:"0 1px 5px #00000012",pointerEvents:"none",opacity:interpolate(frame,[from,from+edge,to-edge,to],[0,1,1,0],{extrapolateLeft:"clamp",extrapolateRight:"clamp"}),scale:interpolate(frame,[from,from+edge,to-edge,to],[1.2,0.85,0.85,1],{extrapolateLeft:"clamp",extrapolateRight:"clamp"})}}/>;
};

export const Artwork: React.FC<{kind?:number}> = ({kind=0}) => {
  const sky=["#c0d9d1","#b0d2dc","#e5bea0"][kind];
  const land=["#406e5e","#52829b","#ac6545"][kind];
  return <svg width="100%" height="100%" viewBox="0 0 300 400" preserveAspectRatio="xMidYMid slice">
    <rect width="300" height="400" fill={sky}/><circle cx="213" cy="115" r="50" fill="#ffe2aa"/>
    <path d="M-20 320Q130 50 330 225V420H-20Z" fill={land} opacity=".35"/>
    <path d="M-20 330Q100 150 330 240V420H-20Z" fill={land}/>
    <path d="M-20 370Q140 210 330 330V420H-20Z" fill="#ffffff" opacity=".07"/>
  </svg>;
};

export const AlbumArtwork: React.FC = () => <svg width="100%" height="100%" viewBox="0 0 300 300"><rect width="300" height="300" fill="#cf653b"/><circle cx="205" cy="118" r="112" fill="#f7b569"/><path d="M-20 310 320 170v160H-20Z" fill="#4a6152"/><path d="M-20 340 320 245v90H-20Z" fill="#354b40"/></svg>;
