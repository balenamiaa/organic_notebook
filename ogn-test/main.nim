import std/[httpclient, json, strutils, strformat]


proc makeReq[bodyType: static[string], T](endpoint: string, `method`: static[string], body: T): Response = 
  const 
    address = "127.0.0.1:8080"
    `method` = `method`.toLowerAscii()
  var client = newHttpClient()
  const httpMethod: HttpMethod = when `method` == "get":
    HttpGet
  elif `method` == "post": 
    HttpPost
  elif `method` == "delete": 
    HttpDelete
  else: 
    {.error:"Unknown method: " & `method`.}

  var reqBody = when bodyType == "json":
    %*(body)
  elif bodyType == "string":
    body
  else:
    {.error:"Unknown body type: " & bodyType.}
    
  return client.request(address & "/api" & endpoint, httpMethod)





let createdIdea = makeReq("/ideas", "post")





