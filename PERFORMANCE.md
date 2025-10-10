| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.5 ± 0.0 | 2.4 | 2.8 | 1.16 ± 0.06 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.1 ± 0.1 | 2.0 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.4 ± 0.0 | 2.4 | 2.7 | 1.16 ± 0.03 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.1 ± 0.1 | 2.0 | 2.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.3 | 4.3 | 1.16 ± 0.06 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.1 ± 0.1 | 2.0 | 2.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 92.5 ± 4.7 | 87.1 | 105.4 | 5.28 ± 0.33 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 17.5 ± 0.6 | 15.7 | 21.8 | 1.00 |

