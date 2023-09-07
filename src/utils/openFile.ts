import { open } from '@tauri-apps/api/dialog';  
import { appDataDir, appDir } from '@tauri-apps/api/path';

export default async function openFile(allowedExtension:string[]) {
  const selected = await open({
    multiple: false,
    filters: [{
      name: 'Default',
      extensions: allowedExtension
    }]
  });
  if (Array.isArray(selected)) {
    // user selected multiple files
    return selected[0]
  } else if (selected === null) {
    return ""
    // user cancelled the selection
  } else {
    return selected
    
    // user selected a single file
  }
}
