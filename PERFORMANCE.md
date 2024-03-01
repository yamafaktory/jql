| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.0 ± 0.5 | 19.2 | 29.8 | 7.16 ± 0.40 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.1 | 2.6 | 4.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.0 ± 0.3 | 19.2 | 22.1 | 7.25 ± 0.38 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.8 ± 0.1 | 2.5 | 3.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 20.0 ± 0.3 | 19.4 | 24.1 | 6.96 ± 0.40 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.9 ± 0.2 | 2.6 | 3.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 120.9 ± 5.3 | 114.4 | 167.9 | 6.87 ± 0.48 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 17.6 ± 1.0 | 15.8 | 42.2 | 1.00 |

