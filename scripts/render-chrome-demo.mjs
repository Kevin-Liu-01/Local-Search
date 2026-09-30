#!/usr/bin/env node
// Deterministic HTML frames in an isolated, loopback-only browser.
import assert from 'node:assert/strict';
import { execFile } from 'node:child_process';
import { createServer } from 'node:http';
import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { promisify } from 'node:util';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const output=path.join(root,'site/public/social/2026-09');
const frames=path.join(root,'artifacts/chrome-demo-frames');
const exec=promisify(execFile);
const fps=20;
const allowed=new Map([
  ['/docs/launch/chrome-demo.html','text/html'],
  ['/docs/launch/benchmark-card.html','text/html'],
  ['/docs/launch/fonts/Manrope.ttf','font/ttf'],
  ['/benchmarks/results/local-ab-2026-09-25-bing-expanded.json','application/json'],
]);
const server=createServer(async(req,res)=>{
  const url=new URL(req.url,'http://127.0.0.1');
  if(!allowed.has(url.pathname)){res.writeHead(404);res.end();return;}
  try{res.writeHead(200,{'Content-Type':allowed.get(url.pathname)});res.end(await readFile(path.join(root,url.pathname)));}
  catch{res.writeHead(500);res.end();}
});
await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
const base=`http://127.0.0.1:${server.address().port}/docs/launch/chrome-demo.html`;
if(process.argv.includes('--serve')){
  console.log(`${base}?play`);
  for(const signal of ['SIGINT','SIGTERM'])process.once(signal,()=>server.close());
}else{
  const env=Object.fromEntries(Object.entries(process.env).filter(([key])=>!key.startsWith('AGENT_BROWSER_')));
  const session=`ls-demo-${process.pid}`;
  const args=['--config',path.join(root,'docs/capture-browser.json'),'--namespace',session,'--session',session,'--allowed-domains','127.0.0.1','--idle-timeout','2m'];
  const run=async(...command)=>(await exec('agent-browser',[...args,...command],{env,timeout:60000,maxBuffer:4000000})).stdout;
  const evaluate=async(expression)=>{
    const value=JSON.parse(await run('--json','eval',expression));
    assert.equal(value.success,true,JSON.stringify(value));return value.data.result;
  };
  const checks=[];
  try{
    await mkdir(output,{recursive:true});await mkdir(frames,{recursive:true});
    await run('batch','--bail',`open ${base}`,'set viewport 1280 720','eval document.fonts.ready.then(()=>true)');
    assert.equal(await evaluate('window.READY'),true);
    for(const [name,time] of [['start',0],['loading',.95],['first-output',1.4],['mid-output',2.3],['search',4.2],['read',7.2],['outro',10]]){
      await run('batch','--bail',`eval "renderFrame(${time})"`,`screenshot ${path.join(frames,`${name}.png`)}`);
      const check=await evaluate('layoutCheck()');
      assert.ok(check.font&&check.width===1280&&check.height===720,JSON.stringify(check));
      assert.ok(check.terminalContentFits&&check.commandFits,JSON.stringify(check));
      assert.ok(check.outputLabelFits,JSON.stringify(check));
      assert.equal(check.terminalWidth,check.browserWidth,'Windows must be exactly 50/50');
      if(name==='search'){
        assert.ok(check.lastResultBottom<=check.resultsViewportBottom-4,JSON.stringify(check));
        assert.ok(check.complete&&check.shownLines===check.totalLines&&check.outputAtEnd,JSON.stringify(check));
        const response=await evaluate(`JSON.parse(document.querySelector('#output-body pre').textContent)`);
        assert.equal(response.ok,true);
        assert.equal(response.search.results.length,3);
        assert.deepEqual(response.search.results.map(result=>result.rank),[1,2,3]);
      }
      checks.push({name,time,...check});console.log(`Verified ${name} frame`);
    }
    await run('batch','--bail','eval "renderFrame(2.3)"',`screenshot ${path.join(output,'chrome-demo.png')}`);
    await run('batch','--bail','eval "renderFrame(10)"',`screenshot ${path.join(output,'chrome-cover.png')}`);
    if(!process.argv.includes('--stills')){
      const duration=await evaluate('DURATION');
      for(let first=0;first<duration*fps;first+=20){
        const commands=[];
        for(let frame=first;frame<Math.min(first+20,duration*fps);frame++){
          commands.push(`eval "renderFrame(${frame/fps})"`,`screenshot ${path.join(frames,`frame-${String(frame).padStart(4,'0')}.png`)}`);
        }
        await run('batch','--bail',...commands);
        console.log(`Captured ${Math.min(first+20,duration*fps)}/${duration*fps} frames`);
      }
      await exec('ffmpeg',['-v','error','-y','-framerate',String(fps),'-i',path.join(frames,'frame-%04d.png'),'-frames:v',String(duration*fps),'-an','-c:v','libx264','-crf','18','-pix_fmt','yuv420p','-movflags','+faststart',path.join(output,'chrome-demo.mp4')],{timeout:120000});
      await exec('ffmpeg',['-v','error','-y','-i',path.join(output,'chrome-demo.mp4'),'-filter_complex','fps=15,scale=960:540:flags=lanczos,split[a][b];[a]palettegen=stats_mode=diff[p];[b][p]paletteuse=dither=bayer:bayer_scale=4','-loop','0',path.join(output,'chrome-demo.gif')],{timeout:120000});
    }
    await run('batch','--bail',`open ${base.replace('chrome-demo.html','benchmark-card.html')}`,'eval document.fonts.ready.then(()=>true)');
    assert.equal(await evaluate('window.READY'),true);
    const benchmark=await evaluate(`({width:document.documentElement.scrollWidth,height:document.documentElement.scrollHeight,footerBottom:document.querySelector('footer').getBoundingClientRect().bottom,font:document.fonts.check('24px Manrope')})`);
    assert.ok(benchmark.font&&benchmark.width===1280&&benchmark.height===720&&benchmark.footerBottom<=694,JSON.stringify(benchmark));
    checks.push({name:'benchmark',...benchmark});
    await run('screenshot',path.join(output,'benchmark-clean.png'));
    await writeFile(path.join(output,'chrome-demo-checks.json'),JSON.stringify({dimensions:[1280,720],gifDimensions:[960,540],fps,checks},null,2)+'\n');
  }finally{
    try{await run('close');}finally{await new Promise(resolve=>server.close(resolve));}
  }
}
