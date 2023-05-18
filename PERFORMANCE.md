| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 31.5 ± 1.3 | 29.5 | 38.4 | 14.24 ± 6.34 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.2 ± 1.0 | 1.8 | 27.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 31.5 ± 1.5 | 29.2 | 49.4 | 15.17 ± 3.53 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.1 ± 0.5 | 1.7 | 9.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 31.5 ± 1.4 | 29.2 | 44.1 | 15.39 ± 2.84 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.0 ± 0.4 | 1.7 | 7.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 181.3 ± 3.2 | 175.7 | 212.5 | 7.86 ± 0.78 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 23.1 ± 2.2 | 18.8 | 37.2 | 1.00 |

