# Web APIs in Rust
This is the repository for the LinkedIn Learning course `Web APIs in Rust`. The full course is available from [LinkedIn Learning][lil-course-url].

![lil-thumbnail-url]

## Course Description

<p>Rust is an extremely powerful and often intimidating language, but it’s particularly well-suited for web services. In this course, instructor Tim McNamara shows you how to get started building and deploying web services written in Rust. Learn how to use the Actix Web framework to create a service that can receive a payload via an HTTP POST request, process it and return JSON. Along the way, you’ll also explore the essentials of stateful endpoints, monitoring and observability, telemetry, testing, deployment, and much more.</p><p>This course is integrated with GitHub Codespaces, an instant cloud developer environment that offers all the functionality of your favorite IDE without the need for any local machine setup. With GitHub Codespaces, you can get hands-on practice from any machine, at any time-all while using a tool that you'll likely encounter in the workplace. Check out the "Using GitHub Codespaces with this course" video to learn how to get started.</p>

_See the readme file in the main branch for updated instructions and information._
## Instructions
This repository has branches for each of the videos in the course. You can use the branch pop up menu in github to switch to a specific branch and take a look at the course at that stage, or you can add `/tree/BRANCH_NAME` to the URL to go to the branch you want to access.

## Branches
The branches are structured to correspond to the videos in the course. The naming convention is `CHAPTER#_MOVIE#`. As an example, the branch named `02_03` corresponds to the second chapter and the third video in that chapter. 
Some branches will have a beginning and an end state. These are marked with the letters `b` for "beginning" and `e` for "end". The `b` branch contains the code as it is at the beginning of the movie. The `e` branch contains the code as it is at the end of the movie. The `main` branch holds the final state of the code when in the course.

When switching from one exercise files branch to the next after making changes to the files, you may get a message like this:

    error: Your local changes to the following files would be overwritten by checkout:        [files]
    Please commit your changes or stash them before you switch branches.
    Aborting

To resolve this issue:
	
    Add changes to git using this command: git add .
	Commit changes using this command: git commit -m "some message"

 ## Instructor

Tim McNamara

Rust Software Architect and Author

                            

Check out my other courses on [LinkedIn Learning](https://www.linkedin.com/learning/instructors/tim-mcnamara?u=104).

[0]: # (Replace these placeholder URLs with actual course URLs)

[lil-course-url]: https://www.linkedin.com/learning/web-apis-in-rust
[lil-thumbnail-url]: https://media.licdn.com/dms/image/v2/D4D0DAQEGrD0k-w4voA/learning-public-crop_675_1200/learning-public-crop_675_1200/0/1734134386529?e=2147483647&v=beta&t=S68B1gssPfZkc6iL0PkhDM9sbVhHvqCsH8FPW5slA5w

