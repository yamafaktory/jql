| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.0 ± 0.3 | 19.3 | 22.0 | 7.29 ± 0.47 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 0.2 | 2.4 | 4.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.9 ± 0.3 | 19.2 | 22.0 | 7.64 ± 0.48 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.6 ± 0.2 | 2.3 | 3.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.8 ± 0.6 | 19.1 | 29.9 | 7.50 ± 0.51 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.6 ± 0.2 | 2.3 | 4.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 123.6 ± 5.5 | 116.8 | 199.4 | 6.60 ± 0.36 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.7 ± 0.6 | 17.0 | 22.4 | 1.00 |

