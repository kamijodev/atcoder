function new () {
    cd ~/projects/atcoder
    acc new $1
    cd $1
    mkdir scripts
    touch scripts/a.py
    touch scripts/b.py
    touch scripts/c.py
    touch scripts/d.py
    touch scripts/e.py
    touch scripts/f.py
}

function test () {
    oj t -c "python scripts/${1}.py" -d ./${1}/tests/
}

function submit () {
    cp "./scripts/${1}.py" "./${1}/main.py"
    cd $1
    acc submit main.py
    cd ../
}

alias new=new
alias test