import requests


print("1. Test hosted API")
print("2. Test local API")
testType = int(input("enter test type : "))

if testType == 1:
    BASE = "https://ducklake.herokuapp.com"
elif testType == 2:
    BASE = "http://127.0.0.1:5000"



response = requests.post(BASE + f"/api/create/1+2+3+4+5+6+7+8+9+0")
print(response)
