| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.6 ± 0.1 | 2.5 | 3.6 | 1.23 ± 0.19 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.1 ± 0.3 | 1.5 | 3.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.1 | 2.4 | 3.1 | 1.28 ± 0.21 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.0 ± 0.3 | 1.4 | 4.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 3.2 | 1.32 ± 0.24 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.9 ± 0.3 | 1.4 | 4.2 | 1.00 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 1.195 ± 0.061 | 1.068 | 1.304 | 9.07 ± 0.47 |
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 0.132 ± 0.002 | 0.129 | 0.135 | 1.00 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 1.000 ± 0.051 | 0.907 | 1.106 | 6.36 ± 0.34 |
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 0.157 ± 0.002 | 0.154 | 0.163 | 1.00 |

