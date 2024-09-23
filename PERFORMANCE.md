| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.2 ± 0.7 | 19.5 | 32.9 | 9.59 ± 0.52 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.1 ± 0.1 | 2.0 | 3.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.1 ± 0.3 | 19.5 | 23.3 | 9.53 ± 0.49 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.1 ± 0.1 | 1.9 | 4.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 20.0 ± 0.5 | 19.2 | 31.1 | 9.60 ± 0.47 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.1 ± 0.1 | 1.9 | 2.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.5 ± 4.7 | 115.7 | 140.8 | 6.74 ± 0.30 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.0 ± 0.4 | 17.1 | 24.4 | 1.00 |

