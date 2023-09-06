import { invoke } from "@tauri-apps/api";

const primitivize_to_alpr = (file_path:string)=>{
    invoke<string>('primitivize_to_alpr',{ filePath: file_path })
     .then(console.log)
     .catch(console.error)
}

export default primitivize_to_alpr