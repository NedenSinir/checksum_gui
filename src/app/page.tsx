// app/page.tsx
'use client'
import { Link } from '@chakra-ui/next-js'
import { Tabs, TabList, Tab, TabPanels, TabPanel, Center, TabIndicator, Heading } from '@chakra-ui/react'
import PreClassTab from '../components/PreClassTab'
import PrimitivizationTab from '@/components/PrimitivizationTab'
import ReversionTab from '@/components/ReversionTab'

export default function Page() {
  return (
    <Center my={"2rem"} alignItems={"center"} w={"100%"}>
    <Tabs isFitted minW={"99%"} position="relative" variant="unstyled">
    <TabList>
      <Tab><Heading size={"lg"}>Predefined Classes</Heading></Tab>
      <Tab><Heading size={"lg"}>Primitivization</Heading></Tab>
      <Tab><Heading size={"lg"}>Reversion</Heading></Tab>
    </TabList>
    <TabIndicator
      mt="-1.5px"
      height="2px"
      bg="blue.500"
      borderRadius="1px"
    />
    <TabPanels>
      <TabPanel>
        <PreClassTab></PreClassTab>
      </TabPanel>
      <TabPanel>
        <PrimitivizationTab></PrimitivizationTab>
      </TabPanel>
      <TabPanel>
        <ReversionTab></ReversionTab>
      </TabPanel>
    </TabPanels>
  </Tabs>
  </Center>
  )
}