import{_ as m}from"./nuxt-link.d26954c9.js";import{n as f,o as s,c as a,a as r,F as d,x,P as u,d as i,t as _,v as h,w as v,u as p,b as g,q as k,y as $}from"./entry.6e79f4bc.js";const b={class:"flex justify-center"},B={class:"flex bg-cyan-950 border-2 border-cyan-900 py-1 px-3 rounded-lg font-mono text-sm"},C=r("div",{class:"my-auto"}," > ",-1),N={class:"p-1 mx-1"},R=f({__name:"NavigationBar",setup(y){function l(t){const o=t.split("/").filter(n=>n!==""),c=[["/","cytos"]];let e="";for(let n of o)e+="/"+n,c.push([e,n]);return c}return(t,o)=>{const c=m;return s(),a("div",b,[r("div",B,[C,(s(!0),a(d,null,x(l((t._.provides[u]||t.$route).path),(e,n)=>(s(),a("div",N,[n===l((t._.provides[u]||t.$route).path).length-1?(s(),a(d,{key:0},[i(_(e[1]),1)],64)):(s(),h(c,{key:1,to:e[0]},{default:v(()=>[i(_(e[1]),1)]),_:2},1032,["to"]))]))),256))])])}}}),S={class:"min-h-screen"},V={class:"container mx-auto h-full text-cyan-50 pb-8 pt-3 px-3 lg:px-20"},P={class:"text-xl font-bold text-center"},w={class:"mt-3"},E={class:"mt-5"},q=f({__name:"default",setup(y){const t=$().public.VERSION||"";return(o,c)=>{const e=m;return s(),a("div",S,[r("div",V,[r("div",P,[(o._.provides[u]||o.$route).path==="/"?(s(),a(d,{key:0},[i(" cytos v"+_(p(t)),1)],64)):(s(),h(e,{key:1,to:"/"},{default:v(()=>[i("cytos v"+_(p(t)),1)]),_:1}))]),r("div",w,[g(R)]),r("div",E,[k(o.$slots,"default")])])])}}});export{q as default};
