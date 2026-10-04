import {pathToFileURL} from 'node:url';
const {atEasel}=await import(pathToFileURL(process.argv[3]+ '/harness/painter/easel-client.ts'));
import readline from 'node:readline';
const studio=process.argv[2];console.log('ready');
for await(const line of readline.createInterface({input:process.stdin})){console.log('invoked');try{console.log(JSON.stringify({reply:await atEasel(studio,['look','--mode','mirror','--size','2000'],undefined)}))}catch(e){console.log(JSON.stringify({error:String(e)}))}break;}
