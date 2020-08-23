from subprocess import Popen, PIPE
import os

class TestComapreException(Exception):
    pass

all_pass = True

def holla(message, raise_exc = False):

    all_pass = False
    if raise_exc:
        raise TestComapreException(message)
    else:
        print(message)

repo_path = os.path.abspath(__file__).rsplit('tests/', 1)[0]
test_path = repo_path + 'tests/'

def spec_tests(sql_filepath='spec_sql.txt'):
    ''' Tests from spec document '''

    spec_commands = (
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 1_a=1                 -f {test_path}one_var_sql.txt ''',    #1
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1       -v b=2      -f {test_path}two_var_sql.txt ''',    #2
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 1_a=1     -v 1_a=2    -f {test_path}one_var_sql.txt ''',    #3
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1       -v b=2      -f {test_path}two_var_sql.txt ''',    #4
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 1_a='1'               -f {test_path}one_var_sql.txt ''',    #5
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 1_a='"1"'             -f {test_path}one_var_sql.txt ''',    #6
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 1_a=                  -f {test_path}one_var_sql.txt ''',    #7
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 1_a==1                -f {test_path}one_var_sql.txt ''',    #8
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v '1_a=a'=1             -f {test_path}one_var_sql.txt ''',    #9 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 1_a='a=1'             -f {test_path}one_var_sql.txt ''',    #10
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v '1_a=1'               -f {test_path}one_var_sql.txt ''',    #11
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a='$(b)'  -v b=2      -f {test_path}two_var_sql.txt ''',    #12
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 1_a'='1               -f {test_path}one_var_sql.txt ''',    #13
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1                   -f {test_path}trailing_data_sql.txt ''',  #14
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1                   -f {test_path}no_vars_sql.txt ''',    #15
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1                   -f {test_path}no_vars_sql.txt ''',    #16

        )

    spec_expected_results = (  
        '1',           #1
        '1 2',         #2
        '2',           #3
        '1 2',         #4
        '1',           #5
        '"1"',         #6
        '',            #7
        '=1',          #8
        'a=1',         #9   
        'a=1',         #10  
        '1',           #11  
        '$(b) 2',      #12
        '1',           #13  
        'a1a 1$(',     #14
        'aaa',         #15
        'aaa',         #16
        )


    for idx, (test_cmd, expected_res) in enumerate(zip(spec_commands, spec_expected_results)):
        
        res, err = Popen(test_cmd, shell=True, stdout=PIPE, stderr=PIPE).communicate()
        res = res.decode('utf8').strip('\n')
        if res != expected_res:
            holla(f'\n\nTest {idx+1} - {test_cmd}\nExpected:\n{expected_res}\nGot:\n{res}\nErr:\n{err}')
        else:
            print (f'{idx+1} OK')
    
    spec_commands_causing_err = (
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1                                       -f {test_path}two_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1 a=2                                   -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a                                         -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1 a                                     -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a -v a=1                                  -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1 -v a=2 -v a                           -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1 b                                     -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -va=1                                        -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v '"a=a"'=1                                 -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v 'a a'=1                                   -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v '$(a)'=1 -v a=2                           -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=2 -v '$(a)'=1                           -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v '!@#$%^&*()1234567890"/,[]{{}}()~`;-_+'=1   -f {test_path}one_var_sql.txt ''',
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho -v a=1 2                                     -f {test_path}one_var_sql.txt ''', 
        f''' {repo_path}/target/x86_64-unknown-linux-musl/release/clotho                                              -f {test_path}doesnotexist.sql ''', 
        )


    for idx, expected_fail_cmd in enumerate(spec_commands_causing_err):
       
        res, err = Popen(expected_fail_cmd.split(), stdout=PIPE, stderr=PIPE).communicate()
        if not err:
            holla(f'\nTest {idx+1}\nExpected error on command:\n{expected_fail_cmd}\nGot:\n{res}\n')
        else:
            print (f'Test {idx+1} raised error - OK')


if __name__ == '__main__':

    spec_tests()
    
    if not all_pass: 
        exit(1)
