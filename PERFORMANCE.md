| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.5 ± 0.0 | 2.4 | 2.7 | 1.17 ± 0.04 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.1 ± 0.1 | 2.0 | 2.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.4 ± 0.1 | 2.4 | 3.8 | 1.15 ± 0.04 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.1 ± 0.0 | 2.0 | 2.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.4 ± 0.1 | 2.3 | 4.8 | 1.15 ± 0.05 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.1 ± 0.0 | 2.0 | 2.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 92.4 ± 5.4 | 87.0 | 168.7 | 5.33 ± 0.36 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 17.3 ± 0.6 | 15.5 | 20.8 | 1.00 |

