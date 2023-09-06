"use client"
import { Button, Center,Text, VStack } from "@chakra-ui/react"
import { invoke } from '@tauri-apps/api/tauri'
import Greet from "../api/greet"
import GenerateApc from "@/api/generate_apc"
import CompressToApc from "@/api/compress_to_apc"
import DecompressFromApc from "@/api/decompress_from_apc"

const PreClassTab = () =>{

    return(


        <>
        <VStack>
        <Greet/>
        <Text> (default)</Text>
        <Button onClick={GenerateApc}  variant={"outline"} colorScheme='blue'>Generate Alper&apos;s Predefined Classes</Button>
        <Button onClick={CompressToApc}  variant={"outline"} colorScheme='blue'>Compress to APC</Button>
        <Button onClick={DecompressFromApc}  variant={"outline"} colorScheme='blue'>Decompress from APC</Button>

        </VStack>   
        </>
    )

}

export default PreClassTab