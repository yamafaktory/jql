| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 26.9 ± 1.2 | 25.2 | 39.8 | 13.64 ± 4.25 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.0 ± 0.6 | 1.6 | 9.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 26.3 ± 1.1 | 25.0 | 35.5 | 14.85 ± 3.18 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.8 ± 0.4 | 1.5 | 7.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 26.3 ± 1.2 | 24.8 | 37.3 | 14.14 ± 4.94 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.9 ± 0.6 | 1.5 | 16.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 154.8 ± 4.5 | 150.0 | 189.7 | 7.64 ± 1.13 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 20.3 ± 2.9 | 16.7 | 53.5 | 1.00 |

