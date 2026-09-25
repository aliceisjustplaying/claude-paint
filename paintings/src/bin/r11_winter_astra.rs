//! The Ford Before Daybreak — an original winter landscape, from knowledge only.
//! Every motif below is drawn for this painting; the engine supplies paint and brushes.
use paint::{Canvas, Gesture, Held, Mask, Paint, Rgb, Rng, Shape, Stipple, Style, Tool,
    Mix, gradient, hex, smoothstep};
use paint::color::mix as lerp;
use paint::noise::Fbm;
use paint::style::Apply;
use paintings::run::{Finish, Run};
type Pt = (f32, f32);

fn line(c: &mut Canvas, pts: &[Pt], width: f32, end: f32, p: Paint, seed: u64) {
    if pts.len() < 2 { return; }
    let tool = Tool { point: 0.88, push: 0.015, pickup: 0.035, ragged: 0.16,
        run: 280.0, ..Tool::round_sable(width.max(0.22) * 1.32) };
    let a = tool.pressure_for(width);
    let b = tool.pressure_for(end.max(0.05));
    let mut held = Held::new(tool, seed);
    held.load(p, 0.85);
    c.drag(&mut held, &Gesture::new(pts.to_vec()).pressure(a,b).ramps(0.015,0.12).shake(0.08), None);
}
fn dab(c: &mut Canvas, at: Pt, w: f32, p: Paint, seed: u64) {
    let mut b = Held::new(Tool::stippler(w), seed);
    b.load(p,0.65);
    c.touch(&mut b,&paint::Touch::at(at.0,at.1).pressure(0.7),None);
}
fn patch(c: &mut Canvas, st: &Style, pts: &[Pt], col: Rgb, seed: u64) {
    let m = Mask::from_shape(c.frame(), Shape::new().poly(pts));
    c.work(&m, &st.body().color(move |_,_| col).coverage(3.2).length(6.0,18.0)
        .angle(|_,_|-0.18).cross(0.22).curve(0.07,0.25).medium(0.12).clip(true), seed);
}
fn bank(x: f32) -> f32 {
    417.0 + 14.0*(x/133.0).sin() + 9.0*(x/62.0+0.4).sin()
}
fn stream(y:f32) -> (f32,f32) {
    let t=((y-419.0)/281.0).clamp(0.0,1.0);
    let mid=534.0 + 125.0*t + 83.0*(t*6.0-0.8).sin()*t;
    (mid, 2.0+92.0*t.powf(1.55))
}
fn snow(x:f32,y:f32)->Rgb {
    let n=Fbm::new(81,4,120.0);
    let t=((y-408.0)/285.0).clamp(0.0,1.0);
    let shade=0.30+0.24*t+0.13*(x*0.011+y*0.025).sin()+0.18*n.get(x,y);
    lerp(hex("#d9d6c9"),hex("#929cab"),shade.clamp(0.0,0.86),Mix::Pigment)
}

// Explicit oak architecture. The long low bough leans over the ford, while
// the upper leader has been broken. Fine growth inherits each bough's direction.
fn oak_boughs() -> Vec<(Vec<Pt>,f32)> {
    vec![
      (vec![(215.,600.),(221.,554.),(215.,504.),(232.,456.),(230.,411.),(249.,370.),(243.,322.),(258.,289.)],25.),
      (vec![(223.,513.),(189.,483.),(172.,439.),(141.,406.),(131.,361.),(105.,325.)],14.),
      (vec![(172.,439.),(120.,431.),(83.,403.),(50.,398.),(17.,372.)],8.5),
      (vec![(141.,406.),(160.,365.),(153.,328.),(166.,295.)],6.5),
      (vec![(131.,361.),(95.,349.),(65.,322.),(43.,319.)],5.),
      (vec![(232.,458.),(277.,428.),(305.,389.),(349.,376.),(388.,351.),(427.,344.)],12.),
      (vec![(306.,389.),(309.,350.),(329.,318.),(326.,287.)],6.5),
      (vec![(349.,376.),(382.,394.),(425.,386.),(455.,398.)],5.8),
      (vec![(230.,411.),(202.,373.),(204.,335.),(187.,302.),(194.,267.)],10.),
      (vec![(203.,335.),(227.,300.),(222.,267.),(236.,230.)],5.5),
      (vec![(249.,371.),(281.,338.),(282.,300.),(298.,274.),(294.,247.)],8.),
      (vec![(282.,300.),(314.,274.),(332.,242.),(356.,228.)],4.4),
      (vec![(243.,322.),(242.,295.),(254.,272.)],6.),
    ]
}
fn twig(c:&mut Canvas, p:Pt, angle:f32, len:f32, width:f32, depth:u32, ink:Paint, r:&mut Rng) {
    let bend=r.range(-0.22,0.22);
    let q=(p.0+angle.cos()*len*0.53,p.1+angle.sin()*len*0.53);
    let end=(q.0+(angle+bend).cos()*len*0.47,q.1+(angle+bend).sin()*len*0.47);
    line(c,&[p,q,end],width,width*0.24,ink,r.next_u64());
    if depth==0 { return; }
    twig(c,end,angle+bend+r.range(-0.24,0.23),len*r.range(0.52,0.79),width*0.65,depth-1,ink,r);
    let side=if r.f()<0.5 {-1.0} else {1.0};
    twig(c,q,angle+side*r.range(0.43,0.91),len*r.range(0.38,0.66),width*0.53,depth-1,ink,r);
}
fn oak(c:&mut Canvas,st:&Style,r:&mut Rng) {
    let dark=st.palette.paint(hex("#39352f"),0.18);
    let gray=st.palette.paint(hex("#66625a"),0.25);
    let light=st.palette.paint(hex("#8a8270"),0.22);
    let fine=st.palette.paint(hex("#59554e"),0.24);
    let boughs=oak_boughs();
    for (pts,w) in &boughs {
        for i in 0..pts.len()-1 {
            let t=i as f32/(pts.len()-1) as f32;
            let t1=(i+1) as f32/(pts.len()-1) as f32;
            line(c,&pts[i..=i+1],w*(1.0-0.86*t),w*(1.0-0.86*t1),dark,r.next_u64());
        }
    }
    // Bark follows the twist of the wood, never crosswise ladder hatching.
    for (pts,w) in &boughs {
        if *w<6.0 {continue;}
        for j in 0..3 {
            let off=(*w)*(j as f32-0.2)*0.12;
            let pp:Vec<Pt>=pts.iter().enumerate().map(|(i,&(x,y))|(x+off*(1.0-i as f32/pts.len() as f32),y)).collect();
            line(c,&pp,w*0.11,w*0.016,if j==2 {light} else {gray},r.next_u64());
        }
    }
    for (pts,w) in &boughs {
        let z=pts.len(); let p=pts[z-1]; let a=pts[z-2];
        let angle=(p.1-a.1).atan2(p.0-a.0);
        if *w>20.0 { // one ragged, dead leader, not a twig brush
            line(c,&[p,(p.0-1.4,p.1-9.0)],1.8,0.4,light,r.next_u64());
            line(c,&[(p.0-3.0,p.1+3.0),(p.0-5.0,p.1-7.0)],1.3,0.2,dark,r.next_u64());
        } else {
            twig(c,p,angle, r.range(19.0,36.0),1.25,4,fine,r);
        }
        for i in 1..z-1 {
            let p=pts[i]; let a=pts[i-1];
            let ang=(p.1-a.1).atan2(p.0-a.0);
            if r.f()<0.82 {
                let s=if i%2==0 {1.0} else {-1.0};
                twig(c,p,ang+s*r.range(0.6,1.2),r.range(18.,40.),1.05,3,fine,r);
            }
        }
    }
    // Buttresses spread sideways and disappear into snow rather than ending in caps.
    for pts in [vec![(221.,555.),(207.,584.),(186.,601.),(146.,612.)],
                vec![(222.,569.),(235.,591.),(266.,602.),(292.,608.)],
                vec![(216.,586.),(201.,608.),(185.,620.)]] {
        line(c,&pts,12.,0.4,dark,r.next_u64());
        let pp:Vec<_>=pts.iter().map(|&(x,y)|(x+1.,y-1.5)).collect();
        line(c,&pp,2.5,0.1,gray,r.next_u64());
    }
    let white=st.palette.paint(hex("#c6c8c3"),0.12);
    for (pts,w) in &boughs {
        for (i,seg) in pts.windows(2).enumerate() {
            let dx=seg[1].0-seg[0].0; let dy=seg[1].1-seg[0].1;
            if dx.abs()>dy.abs()*1.25 && *w>4.0 {
                let off=w*(1.0-i as f32/pts.len() as f32)*0.35;
                let a=(seg[0].0+dx*0.14,seg[0].1+dy*0.14-off);
                let b=(seg[0].0+dx*0.83,seg[0].1+dy*0.83-off*0.7);
                line(c,&[a,b],1.5,0.45,white,r.next_u64());
            }
        }
    }
    // Split knot and small bark fissures concentrated on the near trunk.
    line(c,&[(219.,532.),(213.,540.),(215.,554.),(220.,559.)],2.2,0.3,dark,r.next_u64());
    for _ in 0..46 {
        let y=r.range(474.,587.); let x=218.+r.range(-6.,7.)+3.*(y*0.09).sin();
        line(c,&[(x,y),(x+r.range(-1.5,1.5),y+r.range(2.,10.))],r.range(0.3,0.8),0.12,dark,r.next_u64());
    }
}
fn fir(c:&mut Canvas, st:&Style, x:f32, base:f32, h:f32, col:Rgb, r:&mut Rng) {
    let p=st.palette.paint(col,0.3);
    line(c,&[(x,base),(x-0.7,base-h)],h*0.022,0.14,p,r.next_u64());
    let count=(h*0.7) as usize;
    for i in 1..count {
        let t=i as f32/count as f32;
        let y=base-h+h*t;
        let reach=h*0.24*t.powf(0.85)*r.range(0.66,1.10);
        for s in [-1.,1.] {
            let end=(x+s*reach,y+h*0.08*t);
            line(c,&[(x,y-1.),(x+s*reach*0.5,y+h*0.025),end],(h*0.033*t).max(0.45),0.16,p,r.next_u64());
        }
    }
}
fn traveler(c:&mut Canvas,st:&Style,r:&mut Rng) {
    // A turned back, a cape opening above two boots and one hand on a staff.
    let dark=st.palette.paint(hex("#373d40"),0.10);
    let warm=st.palette.paint(hex("#6d635b"),0.12);
    patch(c,st,&[(593.,477.),(598.,476.),(602.,482.),(604.,493.),(607.,502.),
        (600.,505.),(588.,503.),(590.,490.),(589.,483.)],hex("#454950"),770);
    line(c,&[(594.,501.),(593.,508.),(590.,509.)],2.2,1.5,dark,r.next_u64());
    line(c,&[(600.,501.),(601.,507.),(605.,508.)],2.1,1.4,dark,r.next_u64());
    dab(c,(595.,474.),5.6,dark,r.next_u64());
    line(c,&[(591.,475.),(599.,475.5)],1.1,0.8,dark,r.next_u64());
    line(c,&[(600.,481.),(605.,488.),(610.,489.)],2.6,1.0,warm,r.next_u64());
    line(c,&[(610.,484.),(609.,509.)],0.75,0.55,dark,r.next_u64());
    let rim=st.palette.paint(hex("#96928a"),0.25);
    line(c,&[(597.,479.),(600.,483.),(601.,491.)],0.65,0.15,rim,r.next_u64());
    line(c,&[(594.,485.),(593.,495.),(591.,502.)],0.6,0.3,dark,r.next_u64());
    line(c,&[(598.,488.),(599.,499.)],0.5,0.1,rim,r.next_u64());
}

fn main() {
    let o=Run::new("r11_winter_astra");
    let mut st=Style::friedrich();
    st.width_mm=640.0;
    st.ground[2].color=hex("#d4ccba");
    st.ground[2].um=48.0;
    st.ground[2].apply=Apply::Roller;
    st.mix_jitter=0.015;
    st.relief=(0.032,0.004);
    let mut c=o.canvas(||st.prepare(o.width,1.43,o.seed));
    let mut r=Rng::new(o.seed+117);
    let f=c.frame();
    let all=Mask::from_fn(f,|_,_|1.0);
    if o.stage("drawing",&mut c,&mut r) {
        let pencil=paint::graphite::Lead::graphite(0.42);
        for (pts,_) in oak_boughs() {
            let mark=paint::graphite::hand_line(&pts,&[0.18,0.28],false,false,0.12,r.next_u64());
            c.draw(&pencil,&mark,0.05,r.next_u64());
        }
        let pts:Vec<_>=(0..36).map(|i|{let y=420.+i as f32*8.;(stream(y).0,y)}).collect();
        let mark=paint::graphite::hand_line(&pts,&[0.14],true,false,0.1,42);
        c.draw(&pencil,&mark,0.04,43);
        c.fix_drawing(None);
    }
    if o.stage("sky",&mut c,&mut r) {
        let n=Fbm::new(17,4,235.0);
        let sky=move |x:f32,y:f32| {
            let stops=[(0.0,hex("#697889")),(0.37,hex("#969ca7")),
                (0.70,hex("#b8b4b7")),(1.0,hex("#e0cbb2"))];
            let t=(y/426.0+0.035*n.get(x*0.65,y)).clamp(0.,1.);
            let base=gradient(&stops,t,Mix::Pigment);
            let glow=(-(x-660.).powi(2)/85000.).exp()*smoothstep(190.,417.,y)*0.27;
            lerp(base,hex("#e5d3b6"),glow,Mix::Pigment)
        };
        let pal=st.palette.only(&["lead white","cobalt blue","yellow ochre","red earth","bone black"]);
        c.work(&all,&st.broad().palette(&pal).color(sky).coverage(3.7).length(65.,140.)
            .angle(|_,_|0.04).cross(0.16).curve(0.07,0.4).drift(0.14,220.)
            .medium(0.25).load(0.6).jitter(0.004,0.002),101);
        let sm=Mask::from_fn(f,|_,y|1.-smoothstep(414.,439.,y));
        c.stipple(&sm,&Stipple::new(Tool::stippler(2.5)).mixed(&pal,0.40).color(sky)
            .coverage(|_,_|2.6).dips(24,0.36,0.6).jitter(0.002,0.001),102);
        c.dry();
    }
    if o.stage("distance",&mut c,&mut r) {
        let n=Fbm::new(53,4,161.0);
        for (k,col) in ["#aaaeb1","#929da5","#7f8d97"].iter().enumerate() {
            let crest=f.per_column(move |x| 382.+k as f32*16. + 13.*(x/150.+k as f32*1.2).sin()+9.*n.get(x,25.*k as f32));
            let m=Mask::from_fn(f,move|x,y|smoothstep(crest(x)-2.,crest(x)+3.,y));
            let col=hex(col);
            c.work(&m,&st.body().color(move |x,y|lerp(col,hex("#c4c3bc"),smoothstep(395.,452.,y)*0.8+0.02*n.get(x,y),Mix::Pigment))
                .coverage(3.0).length(15.,42.).angle(|_,_|0.2).cross(0.25).curve(0.07,0.3).medium(0.3).clip(true),200+k as u64);
        }
        // Small, low church: no gigantic Gothic apparition.
        let p=st.palette.paint(hex("#929799"),0.25);
        line(&mut c,&[(684.,401.),(684.,376.)],5.3,4.1,p,r.next_u64());
        patch(&mut c,&st,&[(680.,378.),(683.,365.),(688.,378.)],hex("#929698"),214);
        line(&mut c,&[(676.,400.),(696.,400.)],7.5,6.0,p,r.next_u64());
        line(&mut c,&[(674.,395.),(686.,389.),(700.,396.)],1.8,1.0,p,r.next_u64());
        line(&mut c,&[(683.5,367.),(683.5,362.)],0.5,0.2,p,r.next_u64());
        for _ in 0..100 {
            let x=r.range(5.,995.); let y=425.+6.*(x/63.).sin();
            let h=r.range(3.,15.)*(0.6+0.4*((x-520.).abs()/520.));
            fir(&mut c,&st,x,y,h,hex("#929c9e"),&mut r);
        }
        c.dry();
        let mist=Mask::from_fn(f,|_,y|smoothstep(397.,420.,y)*(1.-smoothstep(442.,460.,y)));
        c.stipple(&mist,&Stipple::new(Tool::stippler(3.0)).mixed(&st.palette,0.55)
            .color(|_,_|hex("#c7c7c0")).coverage(|_,y|1.8*(-((y-429.)/20.).powi(2)).exp())
            .aim(false).fade(1.6).dips(24,0.28,0.6),225);
        c.dry();
    }
    if o.stage("snow and ice",&mut c,&mut r) {
        let n=Fbm::new(90,4,49.0);
        let land=Mask::from_fn(f,|x,y|smoothstep(bank(x)-0.8,bank(x)+1.5,y));
        c.work(&land,&st.broad().color(snow).coverage(3.9).length(24.,68.)
            .angle(|x,_|0.13*(x/123.).sin()).cross(0.22).curve(0.095,0.45)
            .medium(0.13).clip(true).jitter(0.008,0.003),300);
        c.dry();
        let ice=Mask::from_fn(f,move|x,y| {
            let (mid,w)=stream(y);
            smoothstep(425.,438.,y)*(1.-smoothstep(w-1.0,w+1.2,(x-mid+2.*n.get(x,y)).abs()))
        });
        c.work(&ice,&st.body().color(move|x,y| {
            let t=((y-425.)/275.).clamp(0.,1.);
            lerp(hex("#b6bdbe"),hex("#586c7b"),0.22+0.65*t+0.09*n.get(x*0.5,y*2.),Mix::Pigment)
        }).coverage(4.).length(18.,64.).angle(|_,_|0.02).cross(0.08).curve(0.035,0.2)
            .medium(0.23).clip(true),305);
        c.dry();
        // Exposed lip of earth below the overhanging near snow banks.
        let earth=st.palette.paint(hex("#666760"),0.2);
        let lip=st.palette.paint(hex("#d3d4c9"),0.10);
        for side in [-1.,1.] {
            for j in 0..27 {
                let y=457.+j as f32*9.; let (m,w)=stream(y); let (m2,w2)=stream(y+8.);
                let x=m+side*w;
                let pts=[(x,y),(m2+side*w2,y+8.)];
                line(&mut c,&pts,0.5+(y-450.)*0.011,0.6,earth,r.next_u64());
                if j%3!=0 {
                    line(&mut c,&[(x-side*2.,y-1.5),(m2+side*(w2-2.),y+6.)],1.0+(y-450.)*0.007,0.3,lip,r.next_u64());
                }
            }
        }
        // Long, unequal gray seams, not a repeating crystalline pattern.
        let seam=st.palette.paint(hex("#596e7b"),0.38);
        let frost=st.palette.paint(hex("#bbc5c8"),0.25);
        for j in 0..55 {
            let y=r.range(453.,696.); let (m,w)=stream(y);
            let x=m+r.range(-0.8,0.8)*w;
            let len=r.range(3.,22.)*(y-418.)/200.;
            line(&mut c,&[(x,y),(x+len*0.6,y+r.range(-1.,1.)),(x+len,y+1.0)],
                r.range(0.35,0.9),0.15,if j%3==0 {seam} else {frost},r.next_u64());
        }
        // A few dark melt openings, confined to the near bend.
        for (x,y,w) in [(669.,646.,17.),(638.,675.,25.),(707.,609.,9.)] {
            line(&mut c,&[(x-w,y),(x,y-1.),(x+w*0.7,y+1.3)],3.1,0.9,seam,r.next_u64());
        }
    }
    if o.stage("woods",&mut c,&mut r) {
        for j in 0..24 {
            let x=775.+j as f32*11.+r.range(-5.,5.);
            let base=448.+(x-780.)*0.115;
            let h=r.range(25.,70.)*(0.8+0.2*((x-880.)/110.).abs());
            fir(&mut c,&st,x,base,h,hex("#667573"),&mut r);
        }
        // A bare riverside sapling: fine cool wood, not a second large oak.
        let p=st.palette.paint(hex("#77817e"),0.3);
        line(&mut c,&[(739.,472.),(735.,432.),(740.,400.),(736.,375.)],2.2,0.3,p,r.next_u64());
        for j in 0..7 {
            let y=393.+j as f32*9.;
            twig(&mut c,(737.,y),if j%2==0 {-2.2} else {-0.9},r.range(14.,25.),0.7,2,p,&mut r);
        }
        oak(&mut c,&st,&mut r);
        c.dry();
    }
    if o.stage("banks and traveler",&mut c,&mut r) {
        // Flat fractured stones sunk in the left bank, individually faceted.
        for (x,y,s) in [(108.,644.,1.0),(340.,626.,0.64),(79.,587.,0.47),(851.,609.,0.75)] {
            let pts=[(-32.,2.),(-22.,-12.),(3.,-17.),(25.,-9.),(33.,4.),(16.,12.),(-17.,10.)];
            let pp:Vec<_>=pts.iter().map(|&(a,b)|(x+a*s,y+b*s)).collect();
            patch(&mut c,&st,&pp,hex("#5e625f"),r.next_u64());
            let top=[(-30.,0.),(-19.,-12.),(2.,-16.),(23.,-8.),(7.,-4.),(-7.,-5.)];
            let pp:Vec<_>=top.iter().map(|&(a,b)|(x+a*s,y+b*s)).collect();
            patch(&mut c,&st,&pp,hex("#c9cbc3"),r.next_u64());
            let p=st.palette.paint(hex("#838780"),0.2);
            line(&mut c,&[(x-5.*s,y-3.*s),(x+8.*s,y+4.*s),(x+24.*s,y+3.*s)],1.1*s,0.2,p,r.next_u64());
        }
        // The slight hollow around the traveler grounds the feet.
        let shadow=st.palette.paint(hex("#8c99a3"),0.34);
        line(&mut c,&[(589.,509.),(603.,510.),(624.,514.)],3.1,0.2,shadow,r.next_u64());
        traveler(&mut c,&st,&mut r);
        // His approach stops at the ford; tracks become smaller into distance.
        let track=st.palette.paint(hex("#9aa5ab"),0.25);
        for j in 0..13 {
            let t=j as f32/12.; let y=514.+t*t*69.; let x=603.+t*41.+4.*(t*5.).sin();
            line(&mut c,&[(x+if j%2==0 {2.} else {-2.},y),(x+1.,y+1.5+t)],1.2+t*1.8,0.4,track,r.next_u64());
        }
        c.dry();
    }
    if o.stage("winter particulars",&mut c,&mut r) {
        let grass=st.palette.paint(hex("#615c4d"),0.20);
        let pale=st.palette.paint(hex("#afa58b"),0.18);
        let shadow=st.palette.paint(hex("#87949c"),0.28);
        // Tufts are clustered at bank breaks and stones, not uniformly scattered.
        for i in 0..210 {
            let (x,y)=if i<95 {
                let y=r.range(481.,693.); let (m,w)=stream(y);
                (m+if i%2==0 {-w-r.range(2.,17.)} else {w+r.range(3.,18.)},y)
            } else {
                (r.range(12.,971.),r.range(579.,696.))
            };
            let (m,w)=stream(y); if (x-m).abs()<w+1. {continue;}
            if i>=95 && r.f()<0.36 {continue;}
            let scale=((y-411.)/270.).clamp(0.1,1.1);
            line(&mut c,&[(x-2.,y+0.7),(x+5.*scale,y+2.)],2.*scale,0.3,shadow,r.next_u64());
            for j in 0..(3+(r.f()*5.) as usize) {
                let len=r.range(5.,22.)*scale;
                let dx=r.range(-7.,9.)*scale;
                let pts=[(x+r.range(-2.,2.),y),(x+dx*0.3,y-len*0.7),(x+dx,y-len)];
                line(&mut c,&pts,r.range(0.35,0.85)*scale,0.08,if j%4==0 {pale} else {grass},r.next_u64());
                if j==0 && i%5==0 {
                    let top=pts[2];
                    for k in 0..3 {dab(&mut c,(top.0+k as f32*0.45,top.1+k as f32*1.1),0.8*scale,grass,r.next_u64());}
                }
            }
        }
        // Broken fallen twig with a fork on the near snow, in front of the oak.
        let wood=st.palette.paint(hex("#555449"),0.2);
        line(&mut c,&[(285.,664.),(310.,654.),(343.,652.),(361.,643.)],2.3,0.4,wood,r.next_u64());
        line(&mut c,&[(329.,653.),(333.,641.),(347.,633.)],1.1,0.2,wood,r.next_u64());
        line(&mut c,&[(310.,654.),(299.,644.)],0.8,0.1,wood,r.next_u64());
        // Sparse buried stems farther away; very little contrast in the air.
        let p=st.palette.paint(hex("#9ba39e"),0.36);
        for _ in 0..70 {
            let x=r.range(15.,950.);let y=r.range(439.,495.);let (m,w)=stream(y);
            if (x-m).abs()<w+5. {continue;}
            line(&mut c,&[(x,y),(x+r.range(-1.,2.),y-r.range(1.,4.))],0.45,0.12,p,r.next_u64());
        }
        c.dry();
    }
    o.finish(&mut c,&mut r,&Finish {varnish:hex("#f1e5ca"),varnish_coats:0.055,
        varnish_vary:0.018,cracks:None,relief:st.relief});
}
