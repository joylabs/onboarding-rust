# onboarding-rust

### First review if you have set up your Joy Labs [development setup](https://honey.is/home/#post/778734)

1.- [Create an SSH key on your local computer.](https://help.github.com/en/enterprise/2.15/user/articles/adding-a-new-ssh-key-to-your-github-account)

2.- [Check if you have git installed, if not install it:](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
```
git --version
```

3.- On the folder you select, clone the master repository with SSH
```
git clone SSH_link
```

4.- Check repository branches and in wich one you are:
```
git branch
```

5.- Create and move to a new branch with your name (this will be your master personal repository):
```
git checkout -b your_name
```

6.- Create and move to a new branch for your exercice:
```
git checkout -b your_name-e1
```

7.- Solve your exercice, follow the structure of the folders and 
*To run the test navigate to that days submission and run*

```
cargo test
```

8.- Once the tests for your exercise pass, run the comprehensive tests for the entire project
*To run the test navigate to the root of onboarding-rust repo and run*

```
./bin/test
```

9.- Add all your changes (you can review your modified files with *git status*):
```
git add .
```

10.-Create your commit:
```
git commit -m "your comments here"

```

11.- Push changes on your branch:
```
git push origin name_of_the_branch
```

12.- On Github make a pull request of your branch and choose a peer to review it (try to choose someone different).  Check that the automated checks on the PR all pass, and fix any that don't.


### Resources

  - [Git tutorial](https://learngitbranching.js.org)

  - [Cool terminal configuaration (Optional)](https://medium.com/the-code-review/make-your-terminal-more-colourful-and-productive-with-iterm2-and-zsh-11b91607b98c)




Find the Wallbreakers curriculum here

```
https://docs.google.com/document/d/1Vu910E7GVCmjwF77sXNT6dOBEJDDqhEM-yWN0cCGOZM/edit?usp=sharing
```

