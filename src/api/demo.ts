
import { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'

export default async function Demo() {
  console.log("naber oldu");
     invoke<string>('demo')
      .then(console.log)
      .catch(console.error)

  // Necessary because we will have to use Greet as a component later.
  
}