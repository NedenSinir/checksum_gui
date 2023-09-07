import { invoke } from "@tauri-apps/api";

const revert_from_alpr = (file_path:string)=>{
    invoke<string>('revert_from_alpr',{ filePath: file_path })
     .then(console.log)
     .catch(console.error)
}

export default revert_from_alpr