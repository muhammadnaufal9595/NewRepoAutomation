<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Send Chat Attachment</name>
   <tag></tag>
   <elementGuidId>c9b0b77d-1ea0-45bf-b971-cfa67d10dff8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;file&quot;,
      &quot;value&quot;: &quot;C:\\Users\\User\\Pictures\\MacPro Kosong.png&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjYxZmEzODg2MmQ5ZDA3Nzk4NGJkYjE3MSIsImRlZmF1bHRDb21wYW55Ijp7Il9pZCI6IjYxZWJhMmM4NTA4MGY0N2IyNWRkYzhmOCJ9LCJnb29nbGVJZCI6IjExMDcxNzUyMTM1OTc2Mzk1MjgzMyIsImVtYWlsIjoibXVoYW1tYWRuYXVmYWw5NTk1QGdtYWlsLmNvbSIsImZ1bGxOYW1lIjoiTXVoYW1tYWQgTXV6YWtraSIsInBob3RvVXJsIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EtL0FPaDE0R2oxekNwNllrMEc5T3FoalR2eWdHQjhGajlfd2R3bXdYWmJtVGZLPXM5Ni1jIiwiYmlvIjoiIiwic3RhdHVzIjoiIiwiY3JlYXRlZEF0IjoiMjAyMi0wMi0wMlQwNzo1Mzo0Mi45ODZaIiwidXBkYXRlZEF0IjoiMjAyMi0wNC0yM1QxNTowODoxMS4xNjZaIiwiX192IjowfSwiaWF0IjoxNjY4ODQzMDQ5LCJleHAiOjE2NzE0MzUwNDl9.GPpKBrg67Wd0jkdky2etZPs046b4FqRS3aB_Hn0Dslk</value>
   </httpHeaderProperties>
   <katalonVersion>7.5.5</katalonVersion>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://stagingapi.cicle.app/api/v1/chats/6378889466ccc7048fd7b0ea/attachments?companyId=61eba2c85080f47b25ddc8f8</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
