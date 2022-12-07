import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

// Send a POST request to create a new user
// The response contains the id of the newly created user
//get_response = WS.sendRequest(findTestObject('Reqres/Get All User Data'))
// Extract the id value from the response
//user_id = WS.getElementPropertyValue(get_response, 'id')
//println((('ID of user ' + username) + ': ') + user_id.toString())
// Send a GET request to retrieve user information by id
response7 = WS.sendRequest(findTestObject('Reqres/Get All User Data'))

WS.verifyResponseStatusCode(response7, 200)

WS.verifyElementPropertyValue(response7, 'data.id', 1)

WS.verifyElementPropertyValue(response7, 'data.email', 'george.bluth@reqres.in')

WS.verifyElementPropertyValue(response7, 'data.first_name', 'George')

WS.verifyElementPropertyValue(response7, 'data.last_name', 'Bluth')

WS.verifyElementPropertyValue(response7, 'support.url', 'https://reqres.in/#support-heading')

WS.verifyElementPropertyValue(response7, 'support.text', '1 To keep ReqRes free, contributions towards server costs are appreciated!')

