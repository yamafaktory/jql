| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.9 ± 0.9 | 31.7 | 42.7 | 15.70 ± 3.05 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.1 ± 0.4 | 1.5 | 9.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.7 ± 0.9 | 31.5 | 42.3 | 15.99 ± 3.78 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 0.5 | 1.5 | 7.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.7 ± 2.0 | 31.7 | 91.4 | 15.70 ± 4.27 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.1 ± 0.6 | 1.5 | 13.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 165.8 ± 2.1 | 163.7 | 218.6 | 1.29 ± 0.03 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 128.5 ± 2.7 | 123.9 | 151.9 | 1.00 |

