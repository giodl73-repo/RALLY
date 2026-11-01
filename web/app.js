const $=id=>document.getElementById(id),examples=JSON.parse($('examples-data').textContent);$('static-help').hidden=true;
const outputs=['report-download','submitted-download','events-download','validation-download','manifest-download'];
let worker,epoch=0,revision=0,serial=0,active=null,ready=false,busy=false,reading=false,startupTimer,computeTimer,fileTimer,fileToken=0,original=null,result=null,canonical=null;
function controls(){$('run').disabled=!ready||busy||reading;for(const id of outputs)$(id).disabled=!result?.ok;$('original-download').disabled=!$('request').value&&original===null;}
function clear(){canonical=result=null;$('results').hidden=true;$('error').hidden=true;controls();}
function cancelFile(){fileToken++;reading=false;clearTimeout(fileTimer);}
function invalidate(){revision++;clear();if(ready)$('status').textContent=busy?'Inputs changed; waiting for previous work.':'Inputs changed. Replay again.';}
function edited(){cancelFile();original=null;invalidate();}
function diagnostic(detail){$('error').hidden=false;$('error-message').textContent=detail.message;$('diagnostic').textContent=JSON.stringify(detail,null,2);}
function engineError(message){epoch++;worker?.terminate();worker=null;ready=busy=false;active=null;clearTimeout(startupTimer);clearTimeout(computeTimer);clear();diagnostic({message});$('retry').hidden=false;$('status').textContent='Engine unavailable. Input and source downloads remain available.';controls();}
function replay(){
 if(!ready||busy||reading)return;clear();const text=$('request').value;if(new TextEncoder().encode(text).length>131072){diagnostic({message:'Request exceeds 128 KiB UTF-8.'});return;}
 busy=true;active={id:++serial,revision};const id=active.id,generation=epoch;controls();$('status').textContent='Replaying in Rust…';computeTimer=setTimeout(()=>{if(generation===epoch&&active?.id===id)engineError('Replay exceeded ten seconds. Retry with retained input.');},10000);
 try{worker.postMessage({type:'replay',id,request:text});}catch{engineError('The worker could not accept this request.');}
}
function render(){
 $('mechanics-status').textContent=`Mechanics status: ${result.status}`;$('run-id').textContent=`Native run ID: ${result.run.run_id}`;$('state').textContent=JSON.stringify(result.final_state,null,2);$('validation').textContent=JSON.stringify(JSON.parse(result.validation_json),null,2);$('frames').textContent=JSON.stringify(result.frames,null,2);$('events').textContent=result.events_jsonl;$('manifest').textContent=JSON.stringify(JSON.parse(result.manifest_json),null,2);$('results').hidden=false;
}
function start(){
 epoch++;const generation=epoch;worker?.terminate();clearTimeout(startupTimer);clearTimeout(computeTimer);ready=busy=false;active=null;clear();$('retry').hidden=true;$('status').textContent='Loading Rust engine…';startupTimer=setTimeout(()=>{if(generation===epoch&&!ready)engineError('Rust did not start within twelve seconds. Retry to load it again.');},12000);
 try{worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});}catch{engineError('This browser could not create the Rust worker.');return;}
 worker.onerror=e=>{e.preventDefault();if(generation===epoch)engineError('The Rust worker failed.');};worker.onmessage=({data})=>{
  if(generation!==epoch)return;if(data.type==='startup-error'){engineError('Rust could not be downloaded or initialized.');return;}if(data.type==='ready'){clearTimeout(startupTimer);ready=true;$('status').textContent='Rust engine ready.';controls();if(revision===0)replay();return;}
  if(!active||data.id!==active.id)return;clearTimeout(computeTimer);const submitted=active.revision;busy=false;active=null;
  if(data.type==='compute-error'){engineError('The worker could not finish replay.');return;}if(submitted!==revision){$('status').textContent='Inputs changed. Replay again.';controls();return;}
  try{result=JSON.parse(data.canonical);}catch{engineError('The worker returned an unreadable report.');return;}canonical=data.canonical;if(!result.ok){diagnostic(result.diagnostic);$('status').textContent='Fix the request and replay again.';controls();return;}render();$('status').textContent='Replay complete. Inspect mechanics findings and exact submitted input.';controls();
 };
}
function download(name,text,type='application/json'){const url=URL.createObjectURL(new Blob([text],{type:type+';charset=utf-8'}));const link=document.createElement('a');link.href=url;link.download=name;link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
function example(name){cancelFile();original=null;$('request').value=examples[name];$('request-file').value='';invalidate();$('request').focus();}
$('request').addEventListener('input',edited);$('workbench').addEventListener('submit',e=>{e.preventDefault();replay();});$('reset').addEventListener('click',()=>example('script'));$('retry').addEventListener('click',start);for(const button of document.querySelectorAll('[data-example]'))button.addEventListener('click',()=>example(button.dataset.example));
$('original-download').addEventListener('click',()=>download('rally-input.json',original??$('request').value));$('report-download').addEventListener('click',()=>{if(result?.ok)download('rally-mechanics-report.json',canonical);});$('submitted-download').addEventListener('click',()=>{if(result?.ok)download('rally-request.json',result.request_json);});$('events-download').addEventListener('click',()=>{if(result?.ok)download('rally-events.jsonl',result.events_jsonl,'application/x-ndjson');});$('validation-download').addEventListener('click',()=>{if(result?.ok)download('rally-validation.json',result.validation_json);});$('manifest-download').addEventListener('click',()=>{if(result?.ok)download('rally-manifest.json',result.manifest_json);});
$('request-file').addEventListener('change',async()=>{
 const file=$('request-file').files[0];if(!file)return;cancelFile();invalidate();const token=fileToken;if(file.size>131072){diagnostic({message:'Choose valid UTF-8 JSON no larger than 128 KiB.'});$('request-file').value='';return;}
 reading=true;controls();fileTimer=setTimeout(()=>{if(token===fileToken){cancelFile();diagnostic({message:'Local read exceeded five seconds. Previous draft retained.'});controls();}},5000);
 try{const text=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(await file.arrayBuffer());if(token===fileToken){$('request').value=text;$('request-file').value='';edited();original=text;}}
 catch{if(token===fileToken)diagnostic({message:'File could not be read as valid UTF-8. Previous draft retained.'});}finally{if(token===fileToken){reading=false;clearTimeout(fileTimer);$('request-file').value='';}controls();}
});start();
