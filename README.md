# blog
A stupid simple blogging engine written in rust. See a live demo at leeworks.dev

# How to use.
1. Create a folder at `/var/www/`

2. Create a file called inside of this folder `.acl.env` This will tell the server where to route what.

3. Fill this `.acl.env` file using the following syntax
`{url/path}=>{local path}`
    eg
```
/=>index.wmd
/aboutme=>pages/aboutme.wmd
/blogs/nov1=>november1.wmd
```
You will be constantly updating this file as you add more pages

4. Write your .wmd files, see the following example for how to use.
```bash
[t]This is a title
[s]This is a subheading
[p]This is a paragraph body
```
images are still a work in progress.
