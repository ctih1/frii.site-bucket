# frii.site storage

a fun little program that lets you save data onto frii.site TXT records

## How to use
1. Create a base domain on frii.site (e.g "storage")
2. Create an API key (https://development.frii.site/api/dashboard)
> Make sure to select "Any domain" as the target domains, and "List domains" and "Register domains" for the permissions
3. Create a .env file
4. Fill in the following:
```
FRII_KEY=''
BASE_DOMAIN="..."
```
5. Run the program and input "d" to download or "u" to upload a file

> [!IMPORTANT]  
> Make sure to use single quotation marks ('') instead of double quotation marks ("") for FRII_KEY. Because the key starts with "$APIV2=", most systems think that $APIV2 is a variable that is being set. Using single quotation marks solves this issue


## Limitations
By default, accounts can only have up to 50 subdomains. 
This program stores 250 chars of Base64 into every TXT record, so the maximum amount of data per account is roughly 9kb