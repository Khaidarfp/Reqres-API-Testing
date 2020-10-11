<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_List_User</name>
   <tag></tag>
   <elementGuidId>9573d1ce-0f38-40f6-840d-d4e44c6b8f95</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;page\&quot;: 2,\n    \&quot;per_page\&quot;: 6,\n    \&quot;total\&quot;: 12,\n    \&quot;total_pages\&quot;: 2,\n    \&quot;data\&quot;: [\n        {\n            \&quot;id\&quot;: 7,\n            \&quot;email\&quot;: \&quot;michael.lawson@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Michael\&quot;,\n            \&quot;last_name\&quot;: \&quot;Lawson\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/follettkyle/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 8,\n            \&quot;email\&quot;: \&quot;lindsay.ferguson@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Lindsay\&quot;,\n            \&quot;last_name\&quot;: \&quot;Ferguson\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/araa3185/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 9,\n            \&quot;email\&quot;: \&quot;tobias.funke@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Tobias\&quot;,\n            \&quot;last_name\&quot;: \&quot;Funke\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/vivekprvr/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 10,\n            \&quot;email\&quot;: \&quot;byron.fields@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Byron\&quot;,\n            \&quot;last_name\&quot;: \&quot;Fields\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/russoedu/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 11,\n            \&quot;email\&quot;: \&quot;george.edwards@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;George\&quot;,\n            \&quot;last_name\&quot;: \&quot;Edwards\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/mrmoiree/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 12,\n            \&quot;email\&quot;: \&quot;rachel.howell@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Rachel\&quot;,\n            \&quot;last_name\&quot;: \&quot;Howell\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/hebertialmeida/128.jpg\&quot;\n        }\n    ],\n    \&quot;ad\&quot;: {\n        \&quot;company\&quot;: \&quot;StatusCode Weekly\&quot;,\n        \&quot;url\&quot;: \&quot;http://statuscode.org/\&quot;,\n        \&quot;text\&quot;: \&quot;A weekly newsletter focusing on software development, infrastructure, the server, performance, and the stack end of things.\&quot;\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.urllist</defaultValue>
      <description></description>
      <id>8e2ef687-2712-46f5-aa9a-c25d9c8bdfb6</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[0].id', 7)

WS.verifyElementPropertyValue(response, 'data[0].email', &quot;michael.lawson@reqres.in&quot;)

WS.verifyElementPropertyValue(response, 'data[0].first_name', &quot;Michael&quot;)

WS.verifyElementPropertyValue(response, 'data[0].last_name', &quot;Lawson&quot;)

WS.verifyElementPropertyValue(response, 'data[0].avatar', &quot;https://s3.amazonaws.com/uifaces/faces/twitter/follettkyle/128.jpg&quot;)

WS.verifyElementPropertyValue(response, 'data[1].id', 8)

WS.verifyElementPropertyValue(response, 'data[1].email', &quot;lindsay.ferguson@reqres.in&quot;)

WS.verifyElementPropertyValue(response, 'data[1].first_name', &quot;Lindsay&quot;)

WS.verifyElementPropertyValue(response, 'data[1].last_name', &quot;Ferguson&quot;)

WS.verifyElementPropertyValue(response, 'data[1].avatar', &quot;https://s3.amazonaws.com/uifaces/faces/twitter/araa3185/128.jpg&quot;)

WS.verifyElementPropertyValue(response, 'data[2].id', 9)

WS.verifyElementPropertyValue(response, 'data[2].email', &quot;tobias.funke@reqres.in&quot;)

WS.verifyElementPropertyValue(response, 'data[2].first_name', &quot;Tobias&quot;)

WS.verifyElementPropertyValue(response, 'data[2].last_name', &quot;Funke&quot;)

WS.verifyElementPropertyValue(response, 'data[2].avatar', &quot;https://s3.amazonaws.com/uifaces/faces/twitter/vivekprvr/128.jpg&quot;)

WS.verifyElementPropertyValue(response, 'data[3].id', 10)

WS.verifyElementPropertyValue(response, 'data[3].email', &quot;byron.fields@reqres.in&quot;)

WS.verifyElementPropertyValue(response, 'data[3].first_name', &quot;Byron&quot;)

WS.verifyElementPropertyValue(response, 'data[3].last_name', &quot;Fields&quot;)

WS.verifyElementPropertyValue(response, 'data[3].avatar', &quot;https://s3.amazonaws.com/uifaces/faces/twitter/russoedu/128.jpg&quot;)

WS.verifyElementPropertyValue(response, 'data[4].id', 11)

WS.verifyElementPropertyValue(response, 'data[4].email', &quot;george.edwards@reqres.in&quot;)

WS.verifyElementPropertyValue(response, 'data[4].first_name', &quot;George&quot;)

WS.verifyElementPropertyValue(response, 'data[4].last_name', &quot;Edwards&quot;)

WS.verifyElementPropertyValue(response, 'data[4].avatar', &quot;https://s3.amazonaws.com/uifaces/faces/twitter/mrmoiree/128.jpg&quot;)

WS.verifyElementPropertyValue(response, 'data[5].id', 12)

WS.verifyElementPropertyValue(response, 'data[5].email', &quot;rachel.howell@reqres.in&quot;)

WS.verifyElementPropertyValue(response, 'data[5].first_name', &quot;Rachel&quot;)

WS.verifyElementPropertyValue(response, 'data[5].last_name', &quot;Howell&quot;)

WS.verifyElementPropertyValue(response, 'data[5].avatar', &quot;https://s3.amazonaws.com/uifaces/faces/twitter/hebertialmeida/128.jpg&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
