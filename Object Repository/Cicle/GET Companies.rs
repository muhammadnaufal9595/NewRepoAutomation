<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET Companies</name>
   <tag></tag>
   <elementGuidId>f4420196-772e-4b15-819d-046d788c665a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjYxZmEzODg2MmQ5ZDA3Nzk4NGJkYjE3MSIsImRlZmF1bHRDb21wYW55Ijp7Il9pZCI6IjYxZWJhMmM4NTA4MGY0N2IyNWRkYzhmOCJ9LCJnb29nbGVJZCI6IjExMDcxNzUyMTM1OTc2Mzk1MjgzMyIsImVtYWlsIjoibXVoYW1tYWRuYXVmYWw5NTk1QGdtYWlsLmNvbSIsImZ1bGxOYW1lIjoiTXVoYW1tYWQgTXV6YWtraSIsInBob3RvVXJsIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EtL0FPaDE0R2oxekNwNllrMEc5T3FoalR2eWdHQjhGajlfd2R3bXdYWmJtVGZLPXM5Ni1jIiwiYmlvIjoiIiwic3RhdHVzIjoiIiwiY3JlYXRlZEF0IjoiMjAyMi0wMi0wMlQwNzo1Mzo0Mi45ODZaIiwidXBkYXRlZEF0IjoiMjAyMi0wNC0yM1QxNTowODoxMS4xNjZaIiwiX192IjowfSwiaWF0IjoxNjY4ODQzMDQ5LCJleHAiOjE2NzE0MzUwNDl9.GPpKBrg67Wd0jkdky2etZPs046b4FqRS3aB_Hn0Dslk</value>
   </httpHeaderProperties>
   <katalonVersion>7.5.5</katalonVersion>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/v1/companies/</restUrl>
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
