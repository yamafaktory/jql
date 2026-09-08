| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.6 ± 0.0 | 2.5 | 2.9 | 1.27 ± 0.22 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.0 ± 0.3 | 1.5 | 4.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.1 | 2.4 | 3.5 | 1.29 ± 0.22 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.0 ± 0.3 | 1.5 | 3.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 3.4 | 1.30 ± 0.19 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.9 ± 0.3 | 1.5 | 3.2 | 1.00 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 1.167 ± 0.052 | 1.059 | 1.264 | 9.00 ± 0.44 |
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 0.130 ± 0.002 | 0.126 | 0.134 | 1.00 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 1.028 ± 0.058 | 0.903 | 1.126 | 6.68 ± 0.38 |
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 0.154 ± 0.002 | 0.151 | 0.158 | 1.00 |

