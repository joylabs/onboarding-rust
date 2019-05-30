# onboarding-rust


[1.- Create an SSH key on your local computer](https://help.github.com/en/enterprise/2.15/user/articles/adding-a-new-ssh-key-to-your-github-account)

2.- [Intsall git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
```
git --version
```

3.- Clone master repository with SSH
```
git clone "SSH link"
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

7.- Solve your exercice
*To run the test navigate to that days submission and run*

```
cargo test
```

7.- Add all your changes (you can review your modified files with *git status*):
```
Git add .
```

8.-Create your commit:
```
git commit -m "your comments here"

```

9.- Push changes on your branch:
```
Git push origin name_of_the_branch
```

10.- On Github make a pull requeset of your branch and choose someone to review it (try to choose someone different):




