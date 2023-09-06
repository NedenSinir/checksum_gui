import { invoke } from '@tauri-apps/api/tauri'

export default async function CompressToApc() {
  console.log("naber oldu");
     invoke<string>('compress_to_apc')
      .then(console.log)
      .catch(console.error)

  // Necessary because we will have to use Greet as a component later.
  
}