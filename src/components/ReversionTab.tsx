import React, { useEffect, useState } from 'react';
import { Button, FormControl, FormErrorMessage, Input, VStack,Text } from '@chakra-ui/react';
import { Field, Form, Formik } from 'formik';
import openFile from '@/utils/openFile';
import primitivize_to_alpr from '@/api/primitivize_to_alpr';
import Demo from '@/api/demo';
import revert_from_alpr from '@/api/revert_from_alpr';

const ReversionTab = () => {

const [selectedFile ,setSelectedFile] = useState("")

    const handleOpenFile = async () =>{
        const filePath = await openFile(["alpr"])
        setSelectedFile(filePath)        
    }
    const handleRevertFromAlpr = async () =>{
        if(!selectedFile) return
        console.log(selectedFile);
        
        revert_from_alpr(selectedFile)     
    }
  return (
    <VStack>

            <Button mt={4} onClick={handleOpenFile}  variant="outline" colorScheme="blue">
              Select File
            </Button>
            <Text>Selected File : {selectedFile}</Text>
            <Button mt={4} onClick={handleRevertFromAlpr} variant="outline" colorScheme="blue">
              Revert alpr file
            </Button>
      

    </VStack>
  );
};

export default ReversionTab;
