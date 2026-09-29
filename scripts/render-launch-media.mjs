#!/usr/bin/env node
// Authored public-safe HTML only. Never attaches to personal Chrome.
import assert from 'node:assert/strict';
import { execFile } from 'node:child_process';
import { createServer } from 'node:http';
import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { promisify } from 'node:util';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const output=path.join(root,'site/public/social/2026-09');
const exec=promisify(execFile);
const scenes=['hero','search','results','read','choice','connect','disconnect','outro','benchmark'];
const allowed=new Map([
 ['/docs/launch/media.html','text/html'],
 ['/docs/launch/fonts/Manrope.ttf','font/ttf'],
 ['/benchmarks/results/local-ab-2026-09-25-bing-expanded.json','application/json'],
]);
const server=createServer(async(req,res)=>{
 const url=new URL(req.url,'http://127.0.0.1');
 if(!allowed.has(url.pathname)){res.writeHead(404);res.end();return;}
 try{res.writeHead(200,{'Content-Type':allowed.get(url.pathname)});res.end(await readFile(path.join(root,url.pathname)));}
 catch{res.writeHead(500);res.end();}
});
await new Promise(r=>server.listen(0,'127.0.0.1',r));
const base=`http://127.0.0.1:${server.address().port}/docs/launch/media.html`;
if(process.argv.includes('--serve')){
 console.log(base);
 for(const signal of ['SIGINT','SIGTERM'])process.once(signal,()=>server.close());
}else{
 const env=Object.fromEntries(Object.entries(process.env).filter(([k])=>!k.startsWith('AGENT_BROWSER_')));
 const session=`lsearch-media-${process.pid}`;
 const args=['--config',path.join(root,'docs/capture-browser.json'),'--namespace',session,'--session',session,'--allowed-domains','127.0.0.1','--idle-timeout','2m'];
 const run=async(...c)=>(await exec('agent-browser',[...args,...c],{env,timeout:60000,maxBuffer:2000000})).stdout;
 const check=async(expression)=>{const r=JSON.parse(await run('--json','eval',expression));assert.equal(r.success,true,JSON.stringify(r));return r.data.result;};
 try{
  await mkdir(output,{recursive:true});
  await run('batch','--bail',`open ${base}`,'set viewport 1200 900','eval document.fonts.ready.then(()=>true)');
  assert.equal(await check('window.READY'),true);
  const checks=[];
  for(const scene of scenes){
   await run('batch','--bail',`eval "showScene('${scene}')"`,`screenshot ${path.join(output,`${scene}.png`)}`);
   const layout=await check(`(()=>{const footer=document.querySelector('.footer').getBoundingClientRect();const c=document.getElementById('content').getBoundingClientRect();return {scene:${JSON.stringify(scene)},font:document.fonts.check('26px Manrope'),width:document.documentElement.scrollWidth,height:document.documentElement.scrollHeight,contentBottom:c.bottom,footerTop:footer.top,footerBottom:footer.bottom,minType:Math.min(...[...document.querySelectorAll('p,h1,h2,.footer,.top small,.barlabel,.result b,.result span,.chip')].map(n=>parseFloat(getComputedStyle(n).fontSize)))};})()`);
   checks.push(layout);
   assert.equal(layout.font,true);
   assert.ok(layout.width<=1200 && layout.height<=900,JSON.stringify(layout));
   assert.ok(layout.footerTop>=layout.contentBottom+12 && layout.footerBottom<=875,JSON.stringify(layout));
   assert.ok(layout.minType>=22,JSON.stringify(layout));
   assert.equal(await check(`Array.from(document.querySelectorAll('.window')).every(w=>{const b=w.querySelector('.body').getBoundingClientRect(),s=w.querySelector('.status');return b.bottom<=(s?s.getBoundingClientRect().top:w.getBoundingClientRect().bottom)-8;})`),true,`${scene}: window content clipped`);
   console.log(`Rendered ${scene}.png`);
  }
  for(const scene of ['hero','benchmark']){
   await run('batch','--bail','set viewport 1080 1350',`eval "showScene('${scene}',true)"`,`screenshot ${path.join(output,`${scene}-portrait.png`)}`);
   const layout=await check(`(()=>{const f=document.querySelector('.footer').getBoundingClientRect(),c=document.getElementById('content').getBoundingClientRect();return {scene:'${scene}-portrait',width:document.documentElement.scrollWidth,contentBottom:c.bottom,footerTop:f.top,footerBottom:f.bottom};})()`);
   assert.ok(layout.width<=1080 && layout.footerTop>=layout.contentBottom+12 && layout.footerBottom<=1310,JSON.stringify(layout));
   checks.push(layout);
   console.log(`Rendered ${scene}-portrait.png`);
  }
  await writeFile(path.join(output,'layout-checks.json'),JSON.stringify(checks,null,2)+'\n');
 }finally{try{await run('close');}finally{await new Promise(r=>server.close(r));}}
 await exec('python3',[path.join(root,'scripts/animate-launch-media.py')],{timeout:120000,maxBuffer:2000000}).then(r=>console.log(r.stdout));
}
