import React, { useEffect, useState } from 'react';
import { Button, FormControl, FormErrorMessage, Input, VStack,Text } from '@chakra-ui/react';
import { Field, Form, Formik } from 'formik';
import openFile from '@/utils/openFile';
import primitivize_to_alpr from '@/api/primitivize_to_alpr';

const PrimitivizationTab = () => {

const [selectedFile ,setSelectedFile] = useState("")

    const handleOpenFile = async () =>{
        const filePath = await openFile()
        setSelectedFile(filePath)        
    }
    const handlePrimitivizeToAlpr = async () =>{
        if(!selectedFile) return
        console.log(selectedFile);
        
        primitivize_to_alpr(selectedFile)     
    }
  return (
    <VStack>

            <Button mt={4} onClick={handleOpenFile}  variant="outline" colorScheme="blue">
              Open File
            </Button>
            <Text>Selected File : {selectedFile}</Text>
            <Button mt={4} onClick={handlePrimitivizeToAlpr} variant="outline" colorScheme="blue">
              Primitivize
            </Button>
      

    </VStack>
  );
};

export default PrimitivizationTab;
