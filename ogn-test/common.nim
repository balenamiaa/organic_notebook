import std/[httpclient, json, strutils, strformat, math, sequtils, sugar]

proc makeReq*(
  endpoint: string, `method`, `return`: static[string], reqBody: string = "",
  headers: HttpHeaders = newHttpHeaders(), files: MultipartData = newMultipartData()
): auto =

  const
    address = "http://127.0.0.1:8080"
    `method` = `method`.toLowerAscii()
  var client = newHttpClient()
  const httpMethod: HttpMethod = when `method` == "get":
    HttpGet
  elif `method` == "post":
    HttpPost
  elif `method` == "delete":
    HttpDelete
  else:
    {.error: "Unknown method: " & `method`.}
    
  let resp = client.request(address & "/api" & endpoint, httpMethod, reqBody, headers, files)

  when `return` == "status":
    resp.code()
  elif `return` == "status,json":
    dump resp.body()
    (
      status: resp.code(),
      json: if resp.code() == Http200: resp.body().parseJson() else: nil
    )
  elif `return` == "status,string":
    dump resp.body()
    (
      status: resp.code(),
      val: if resp.code() == Http200: resp.body() else: ""
    )
  else:
    {.error: "Unknown return type: " & `return`.}


export json, httpclient, strutils, math, sequtils
