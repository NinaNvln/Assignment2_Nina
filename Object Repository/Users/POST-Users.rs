<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST-Users</name>
   <tag></tag>
   <elementGuidId>54fee25a-cb02-4554-91da-c6d25601fabf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;: 8,\n    \&quot;name\&quot;: \&quot;nina\&quot;,\n    \&quot;username\&quot;: \&quot;test\&quot;,\n    \&quot;email\&quot;: \&quot;Sincere@april.biz\&quot;,\n      \&quot;street\&quot;: \&quot;test\&quot;,\n      \&quot;suite\&quot;: \&quot;Apt. 556\&quot;,\n      \&quot;city\&quot;: \&quot;Gwenborough\&quot;,\n      \&quot;zipcode\&quot;: \&quot;92998-3874\&quot;,\n        \&quot;lat\&quot;: \&quot;-37.3159\&quot;,\n        \&quot;lng\&quot;: \&quot;81.1496\&quot;\n     \n    }&quot;,
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
      <webElementGuid>8c422909-ebae-4c4e-8432-13a7c06a8d42</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyResponseStatusCode(response, 201)




WS.verifyElementPropertyValue(response, 'id', '11')
WS.verifyElementPropertyValue(response, 'name', 'nina')
WS.verifyElementPropertyValue(response, 'username', 'test')
WS.verifyElementPropertyValue(response, 'email', 'Sincere@april.biz')
WS.verifyElementPropertyValue(response, 'street', 'test')
WS.verifyElementPropertyValue(response, 'suite', 'Apt. 556')
WS.verifyElementPropertyValue(response, 'city', 'Gwenborough')
WS.verifyElementPropertyValue(response, 'zipcode', '92998-3874')
WS.verifyElementPropertyValue(response, 'lat', '-37.3159')
WS.verifyElementPropertyValue(response, 'lng', '81.1496')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
