| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.6 ± 0.1 | 2.5 | 4.9 | 1.24 ± 0.21 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.1 ± 0.3 | 1.5 | 4.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.6 ± 0.1 | 2.5 | 3.8 | 1.26 ± 0.22 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.1 ± 0.4 | 1.5 | 5.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 3.4 | 1.26 ± 0.21 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.0 ± 0.3 | 1.5 | 3.9 | 1.00 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 1.234 ± 0.057 | 1.089 | 1.321 | 9.07 ± 0.45 |
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 0.136 ± 0.002 | 0.133 | 0.141 | 1.00 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 1.088 ± 0.051 | 0.942 | 1.186 | 6.80 ± 0.33 |
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 0.160 ± 0.002 | 0.157 | 0.163 | 1.00 |

