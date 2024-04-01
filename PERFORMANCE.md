| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.0 ± 0.4 | 19.3 | 24.5 | 7.44 ± 0.53 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 0.2 | 2.3 | 3.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.3 ± 0.6 | 19.3 | 31.3 | 7.39 ± 0.60 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.7 ± 0.2 | 2.4 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 20.2 ± 1.3 | 19.4 | 58.1 | 7.21 ± 0.69 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.8 ± 0.2 | 2.4 | 4.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.2 ± 5.0 | 114.9 | 154.6 | 6.62 ± 0.34 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.3 ± 0.6 | 16.4 | 24.3 | 1.00 |

